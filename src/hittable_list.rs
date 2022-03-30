use crate::{HitRecord, Hittable, Ray};

pub struct HittableList<const CAPACITY: usize> {
    size: usize,
    objects: [Option<Hittable>; CAPACITY],
}

impl<const CAPACITY: usize> HittableList<CAPACITY> {
    pub const fn new() -> Self {
        Self {
            size: 0,
            objects: [None; CAPACITY],
        }
    }
}

impl<const CAPACITY: usize> HittableList<CAPACITY> {
    pub const fn clear(&mut self) {
        self.objects = [None; CAPACITY];
        self.size = 0;
    }

    pub const fn add(&mut self, hittable: Hittable) {
        assert!(self.size < CAPACITY);
        self.objects[self.size] = Some(hittable);
        self.size += 1;
    }

    pub const fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut result: Option<HitRecord> = None;
        let mut closest_so_far = t_max;

        let mut index = 0;
        while index < self.size {
            let hit_record = match &self.objects[index] {
                Some(Hittable::Sphere(ref s)) => s.hit(&r, t_min, closest_so_far),
                _ => None,
            };

            if let Some(hit_record) = hit_record {
                closest_so_far = hit_record.t;
                result = Some(hit_record);
            }

            index += 1;
        }

        result
    }
}
