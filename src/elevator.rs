type Floor = i32;

#[derive(Debug)]
pub enum Event {
    CarArrived(Floor),
    CarMoving(Direction),
    CarDoorOpened,
    CarDoorClosed,
    LobbyButtonPressed(Floor, Direction),
    CarFloorButtonPressed(Floor)
}

#[derive(Debug)]
pub enum Direction {
    Up,
    Down
}

pub fn car_arrived(floor: Floor) -> Event {
    Event::CarArrived(floor)
}

pub fn car_door_opened() -> Event {
    Event::CarDoorOpened
}

pub fn car_door_closed() -> Event {
    Event::CarDoorClosed
}

pub fn car_moving(direction: Direction) -> Event {
    Event::CarMoving(direction)
}

pub fn lobby_button_pressed(floor: Floor, direction: Direction) -> Event {
    Event::LobbyButtonPressed(floor, direction)
}

pub fn car_floor_button_pressed(floor: Floor) -> Event {
    Event::CarFloorButtonPressed(floor)
}