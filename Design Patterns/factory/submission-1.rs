trait Vehicle {
    fn get_type(&self) -> &str;
}

struct Car;

impl Vehicle for Car {
    fn get_type(&self) -> &str {
        "Car"
    }
}

struct Bike;

impl Vehicle for Bike {
    fn get_type(&self) -> &str {
        "Bike"
    }
}

struct Truck;

impl Vehicle for Truck {
    fn get_type(&self) -> &str {
        "Truck"
    }
}

trait VehicleFactory {
    fn create_vehicle(&self) -> Box<dyn Vehicle>;
}

struct CarFactory;

impl VehicleFactory for CarFactory {
    fn create_vehicle(&self) -> Box<dyn Vehicle> {
        Box::new(Car)
    }
}

struct BikeFactory;

impl VehicleFactory for BikeFactory {
    fn create_vehicle(&self) -> Box<dyn Vehicle> {
        Box::new(Bike)
    }
}

struct TruckFactory;

impl VehicleFactory for TruckFactory {
    fn create_vehicle(&self) -> Box<dyn Vehicle> {
        Box::new(Truck)
    }
}
