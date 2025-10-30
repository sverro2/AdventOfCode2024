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

    pub fn push(&mut self, item_location: IVec2, direction: &BotMove) -> IVec2 {
        let next_location = direction.get_next_vec(item_location);

        if self.can_push(next_location, direction) {
            self.push_unchecked(item_location, direction)
        } else {
            item_location
        }
    }

    fn push_unchecked(&mut self, item_location: IVec2, direction: &BotMove) -> IVec2 {
        let next_location = direction.get_next_vec(item_location);

        // Check what content is at the next location
        match &self.contents[next_location.y as usize][next_location.x as usize] {
            // In specific cases moves will branch out
            Content::WideboxLeftPart if matches!(direction, BotMove::Up | BotMove::Down) => {
                self.push_wide_box_vertical(next_location, direction, IVec2::X);
                self.move_item(item_location, next_location);
                next_location
            }
            Content::WideBoxRightPart if matches!(direction, BotMove::Up | BotMove::Down) => {
                self.push_wide_box_vertical(next_location, direction, -IVec2::X);
                self.move_item(item_location, next_location);
                next_location
            }
            // Other cases are a bit more simple
            Content::Robot
            | Content::Box
            | Content::WideboxLeftPart
            | Content::WideBoxRightPart => {
                // Recursively push the item at next location
                self.push_unchecked(next_location, direction);

                // Move our item to the next location
                self.move_item(item_location, next_location);

                next_location
            }
            Content::Empty => {
                // Move our item to the empty space
                self.move_item(item_location, next_location);
                next_location
            }
            Content::Wall => item_location,
        }
    }

    fn push_wide_box_vertical(
        &mut self,
        box_part_location: IVec2,
        direction: &BotMove,
        other_part_offset: IVec2,
    ) {
        // Recursively push both parts of the wide box starting with the immediatally touching part
        self.push_unchecked(box_part_location, direction);

        // And then the other part of the box
        let other_part_location = box_part_location + other_part_offset;
        self.push_unchecked(other_part_location, direction);
    }

    fn can_push(&mut self, location: IVec2, direction: &BotMove) -> bool {
        match &self.contents[location.y as usize][location.x as usize] {
            // In specific cases checks will branch out
            Content::WideboxLeftPart if matches!(direction, BotMove::Up | BotMove::Down) => {
                let next_location = direction.get_next_vec(location);
                self.can_push(next_location, direction)
                    && self.can_push(next_location + IVec2::X, direction)
            }
            Content::WideBoxRightPart if matches!(direction, BotMove::Up | BotMove::Down) => {
                let next_location = direction.get_next_vec(location);
                self.can_push(next_location, direction)
                    && self.can_push(next_location - IVec2::X, direction)
            }
            // Other cases are a bit more simple
            Content::Robot
            | Content::Box
            | Content::WideboxLeftPart
            | Content::WideBoxRightPart => {
                let next_location = direction.get_next_vec(location);
                self.can_push(next_location, direction)
            }
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
