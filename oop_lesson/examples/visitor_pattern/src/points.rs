pub mod points_module {
    use interfaces::Visitor;
    //objects
    #[derive(Default)]
    pub struct Point1D {
        x: i32,
        alpha: f32,
    }
    impl std::fmt::Debug for Point1D {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.debug_struct("Point1D")
                .field("coordinate x value is", &self.x)
                .field("alpha level", &self.alpha)
                .finish()
        }
    }
    impl std::fmt::Debug for Point2D {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.debug_struct("Point2D")
                .field("coordinate x is", &self.x)
                .field("coordinate y is", &self.y)
                .field("alpha level", &self.alpha)
                .finish()
        }
    }
    #[derive(Default)]
    pub struct Point2D {
        x: i32,
        y: i32,
        alpha: f32,
    }
    pub trait MovableObjects {
        fn to_right(&mut self, obj: &impl Visitor);
        fn to_left(&mut self, obj: &impl Visitor);
        fn apply_factor(&mut self, obj: &impl Visitor);
    }

    #[derive(Default)]
    pub struct Moving(Option<i32>); // public
    impl Moving {
        pub fn apply_factor(factor: i32) -> Self {
            Self(Some(factor))
        }
    }
    mod interfaces {

        use super::Moving;
        use super::{MovableObjects, Point1D, Point2D};
        impl MovableObjects for Point1D {
            fn to_right(&mut self, obj: &impl Visitor) {
                obj.move_right_1d(self); // pass self here
            }

            fn to_left(&mut self, obj: &impl Visitor) {
                obj.move_left_1d(self);
            }

            fn apply_factor(&mut self, obj: &impl Visitor) {
                obj.factorize_1d(self);
            }
        }
        impl MovableObjects for Point2D {
            fn to_right(&mut self, obj: &impl Visitor) {
                obj.move_right_2d(self); // pass self here
            }

            fn to_left(&mut self, obj: &impl Visitor) {
                obj.move_left_2d(self);
            }

            fn apply_factor(&mut self, obj: &impl Visitor) {
                obj.factorize_2d(self);
            }
        }
        //overkill
        pub trait Visitor {
            fn move_right_1d(&self, obj: &mut Point1D);
            fn move_right_2d(&self, obj: &mut Point2D);
            fn move_left_1d(&self, obj: &mut Point1D);
            fn move_left_2d(&self, obj: &mut Point2D);
            fn factorize_2d(&self, obj: &mut Point2D);
            fn factorize_1d(&self, obj: &mut Point1D);
        }
        impl Visitor for Moving {
            fn move_right_1d(&self, obj: &mut Point1D) {
                obj.move_right();
            }

            fn move_right_2d(&self, obj: &mut Point2D) {
                obj.move_right();
            }

            fn move_left_1d(&self, obj: &mut Point1D) {
                obj.move_left();
            }

            fn move_left_2d(&self, obj: &mut Point2D) {
                obj.move_left();
            }

            fn factorize_2d(&self, obj: &mut Point2D) {
                if let Some(val) = self.0 {
                    obj.apply_transform(val);
                } else {
                    obj.apply_transform(1_i32);
                }
            }

            fn factorize_1d(&self, obj: &mut Point1D) {
                if let Some(val) = self.0 {
                    obj.apply_transform(val);
                } else {
                    obj.apply_transform(1_i32);
                }
            }
        }
        //get general properties
        trait Object {
            type Coord;
            fn get_coordinates(&self) -> Self::Coord;
            fn reset(&mut self);
            fn make_invisible(&mut self);
            fn set_coordinates(&mut self, coord: Self::Coord);
        }
        impl Object for Point1D {
            type Coord = i32;

            fn get_coordinates(&self) -> Self::Coord {
                self.x
            }
            fn reset(&mut self) {
                self.x = 0;
            }

            fn make_invisible(&mut self) {
                self.alpha = 0.0;
            }

            fn set_coordinates(&mut self, coord: Self::Coord) {
                self.x = coord;
            }
        }
        impl Object for Point2D {
            type Coord = (i32, i32);

            fn get_coordinates(&self) -> Self::Coord {
                (self.x, self.y)
            }

            fn reset(&mut self) {
                self.x = 0;
                self.y = 0;
            }

            fn make_invisible(&mut self) {
                self.alpha = 0.0;
            }

            fn set_coordinates(&mut self, coord: Self::Coord) {
                self.x = coord.0;
                self.y = coord.1;
            }
        }

        //simple operations
        trait MoveObject {
            fn move_right(&mut self);
            fn move_left(&mut self);
            fn reset(&mut self);
        }
        impl MoveObject for Point1D {
            fn move_right(&mut self) {
                let tmp = self.get_coordinates();
                self.set_coordinates(tmp + 1);
            }

            fn move_left(&mut self) {
                let tmp = self.get_coordinates();
                self.set_coordinates(tmp - 1);
            }

            fn reset(&mut self) {
                self.set_coordinates(0_i32);
            }
        }
        impl MoveObject for Point2D {
            fn move_right(&mut self) {
                let tmp = self.get_coordinates();
                self.set_coordinates((tmp.0 + 1, tmp.1 + 1));
            }

            fn move_left(&mut self) {
                let tmp = self.get_coordinates();
                self.set_coordinates((tmp.0 - 1, tmp.1 - 1));
            }

            fn reset(&mut self) {
                self.set_coordinates((0_i32, 0_i32));
            }
        }

        //simple transform
        trait TransformCoord: MoveObject {
            fn apply_transform(&mut self, factor: i32);
        }
        impl TransformCoord for Point1D {
            fn apply_transform(&mut self, factor: i32) {
                self.set_coordinates(self.get_coordinates() * factor);
            }
        }
        impl TransformCoord for Point2D {
            fn apply_transform(&mut self, factor: i32) {
                let mut tmp = self.get_coordinates();
                tmp.0 *= factor;
                tmp.1 *= factor;
                self.set_coordinates(tmp);
            }
        }
    }
    mod implement {}
}
