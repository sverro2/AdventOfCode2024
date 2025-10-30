use glam::IVec2;

use crate::{BotMove, Content};

#[derive(Debug, Clone)]
pub struct Warehouse {
    contents: Vec<Vec<Content>>,
    width: usize,
    height: usize,
}
impl Warehouse {
    pub fn new(contents: Vec<Vec<Content>>, width: usize, height: usize) -> Self {
        Self {
            contents,
            width,
            height,
        }
    }

    pub fn push(&mut self, pusher_location: IVec2, direction: &BotMove) -> IVec2 {
        if self.can_push(pusher_location, direction) {
            self.push_unchecked(pusher_location, direction)
        } else {
            pusher_location
        }
    }

    fn push_unchecked(&mut self, pusher_location: IVec2, direction: &BotMove) -> IVec2 {
        let next_location = direction.get_next_vec(pusher_location);

        // Check what content is at the next location

        match &self.contents[next_location.y as usize][next_location.x as usize] {
            // In specific cases checks will branch out
            Content::WideboxLeftPart if matches!(direction, BotMove::Up | BotMove::Down) => {
                self.push_wide_box_vertical(next_location, direction, IVec2::X)
            }
            Content::WideBoxRightPart if matches!(direction, BotMove::Up | BotMove::Down) => {
                self.push_wide_box_vertical(next_location, direction, -IVec2::X)
            }
            // Other cases are a bit more simple
            Content::Robot
            | Content::Box
            | Content::WideboxLeftPart
            | Content::WideBoxRightPart => {
                // Recursively push the pushable
                let pushable_destination = self.push_unchecked(next_location, direction);

                // Move the box to its new location
                self.move_item(next_location, pushable_destination);

                // Return the position where the pusher should end up (behind the pushable)
                direction.get_prev_vec(pushable_destination)
            }
            Content::Empty => next_location,
            Content::Wall => pusher_location,
        }
    }

    fn push_wide_box_vertical(
        &mut self,
        box_part_location: IVec2,
        direction: &BotMove,
        other_part_offset: IVec2,
    ) -> IVec2 {
        // Recursively push the pushable
        let pushable_destination = self.push_unchecked(box_part_location, direction);

        // Move the box to its new location
        self.move_item(box_part_location, pushable_destination);

        // Move other part of the box as well
        let other_part_location = box_part_location + other_part_offset;
        let other_part_destination = self.push_unchecked(other_part_location, direction);
        self.move_item(other_part_location, other_part_destination);

        // Return the position where the pusher should end up (behind the pushable)
        direction.get_prev_vec(pushable_destination)
    }

    fn can_push(&mut self, pusher_location: IVec2, direction: &BotMove) -> bool {
        let next_location = direction.get_next_vec(pusher_location);

        match &self.contents[next_location.y as usize][next_location.x as usize] {
            // In specific cases checks will branch out
            Content::WideboxLeftPart if matches!(direction, BotMove::Up | BotMove::Down) => {
                self.can_push(next_location, direction)
                    && self.can_push(next_location + IVec2::X, direction)
            }
            Content::WideBoxRightPart if matches!(direction, BotMove::Up | BotMove::Down) => {
                self.can_push(next_location, direction)
                    && self.can_push(next_location - IVec2::X, direction)
            }
            // Other cases are a bit more simple
            Content::Robot
            | Content::Box
            | Content::WideboxLeftPart
            | Content::WideBoxRightPart => self.can_push(next_location, direction),
            Content::Empty => true,
            Content::Wall => false,
        }
    }

    fn move_item(&mut self, source_location: IVec2, target_location: IVec2) {
        let source_value = std::mem::replace(
            &mut self.contents[source_location.y as usize][source_location.x as usize],
            Content::Empty,
        );
        self.contents[target_location.y as usize][target_location.x as usize] = source_value;
    }

    pub fn calc_gps_all_crates(&self) -> usize {
        (0..self.height)
            .flat_map(|row_index| {
                (0..self.width).filter_map(move |column_index| {
                    if matches!(
                        self.contents[row_index][column_index],
                        Content::Box | Content::WideboxLeftPart
                    ) {
                        Some(row_index * 100 + column_index)
                    } else {
                        None
                    }
                })
            })
            .sum()
    }

    #[allow(dead_code)]
    pub fn print(&self) {
        for row in &self.contents {
            for content in row {
                print!(
                    "{}",
                    match content {
                        Content::Box => 'O',
                        Content::Empty => '.',
                        Content::Wall => '#',
                        Content::Robot => '@',
                        Content::WideboxLeftPart => '[',
                        Content::WideBoxRightPart => ']',
                    }
                );
            }
            println!();
        }
    }

    pub fn get_bot_location(&self) -> Option<IVec2> {
        self.contents
            .iter()
            .enumerate()
            .find_map(|(row_index, row)| {
                row.iter().enumerate().find_map(|(col_index, col)| {
                    if matches!(col, Content::Robot) {
                        Some(IVec2::new(col_index as i32, row_index as i32))
                    } else {
                        None
                    }
                })
            })
    }
}
