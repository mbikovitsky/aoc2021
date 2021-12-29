use std::ops::{Sub, SubAssign};

use itertools::Itertools;
use num::{CheckedAdd, CheckedMul, CheckedSub, Integer};
use petgraph::{
    graph::NodeIndex,
    stable_graph::StableDiGraph,
    visit::{depth_first_search, Control, Dfs, DfsEvent, Reversed},
    EdgeDirection::{Incoming, Outgoing},
};

use crate::r#box::Box;

#[derive(Debug, Clone)]
pub struct SwissBox<T: Integer> {
    tree: StableDiGraph<Box<T>, ()>,
}

impl<T: Integer> SwissBox<T> {
    pub fn new(initial: Box<T>) -> Self {
        if initial.is_empty() {
            Default::default()
        } else {
            let mut tree = StableDiGraph::new();
            tree.add_node(initial);
            Self { tree }
        }
    }

    pub fn is_empty(&self) -> bool {
        self.tree.node_count() == 0
    }

    fn root(&self) -> Option<NodeIndex> {
        if self.is_empty() {
            return None;
        }

        Some(
            self.tree
                .externals(Incoming)
                .exactly_one()
                .map_err(|_| "More than one root in the tree")
                .unwrap(),
        )
    }

    fn is_leaf(&self, node: NodeIndex) -> bool {
        self.tree
            .neighbors_directed(node, Outgoing)
            .next()
            .is_none()
    }
}

impl<T: Integer> Default for SwissBox<T> {
    fn default() -> Self {
        Self {
            tree: Default::default(),
        }
    }
}

impl<T: Integer> From<Box<T>> for SwissBox<T> {
    fn from(value: Box<T>) -> Self {
        Self::new(value)
    }
}

impl<T: Integer + Clone> Sub<Box<T>> for SwissBox<T> {
    type Output = Self;

    fn sub(self, rhs: Box<T>) -> Self::Output {
        self - &rhs
    }
}

impl<'a, T: Integer + Clone> Sub<&'a Box<T>> for SwissBox<T> {
    type Output = Self;

    fn sub(mut self, rhs: &'a Box<T>) -> Self::Output {
        self -= rhs;
        self
    }
}

impl<T: Integer + Clone> Sub<SwissBox<T>> for SwissBox<T> {
    type Output = Self;

    fn sub(self, rhs: SwissBox<T>) -> Self::Output {
        self - &rhs
    }
}

impl<'a, T: Integer + Clone> Sub<&'a SwissBox<T>> for SwissBox<T> {
    type Output = Self;

    fn sub(mut self, rhs: &'a SwissBox<T>) -> Self::Output {
        self -= rhs;
        self
    }
}

impl<T: Integer + Clone> SubAssign<Box<T>> for SwissBox<T> {
    fn sub_assign(&mut self, rhs: Box<T>) {
        *self -= &rhs;
    }
}

impl<'a, T: Integer + Clone> SubAssign<&'a Box<T>> for SwissBox<T> {
    fn sub_assign(&mut self, rhs: &'a Box<T>) {
        if self.is_empty() {
            return;
        }

        let mut to_add = vec![];
        let mut to_delete = vec![];

        depth_first_search(
            &self.tree,
            Some(self.root().unwrap()),
            |event| -> Control<()> {
                if let DfsEvent::Discover(node, _) = event {
                    let intersection = self.tree[node].intersect(rhs);

                    // No need to check the children if the bounding box does not intersect
                    // the deletion box
                    if intersection.is_empty() {
                        return Control::Prune;
                    }

                    if !self.is_leaf(node) {
                        return Control::Continue;
                    }

                    if intersection == self.tree[node] {
                        // This whole node should be deleted, and not split.
                        to_delete.push(node);
                        return Control::Continue;
                    }

                    // Split the node, and remember where the splits should be added
                    for slice in self.tree[node].subtract_split(rhs) {
                        to_add.push((node, slice));
                    }
                }

                Control::Continue
            },
        );

        for (parent, slice) in to_add {
            let child = self.tree.add_node(slice);
            self.tree.add_edge(parent, child, ());
        }

        for node in to_delete {
            let mut ancestors = vec![];

            let reversed = Reversed(&self.tree);
            let mut dfs = Dfs::new(&reversed, node);
            while let Some(ancestor) = dfs.next(&reversed) {
                if self.tree.neighbors_directed(ancestor, Outgoing).count() <= 1 {
                    ancestors.push(ancestor);
                } else {
                    break;
                }
            }

            for ancestor in ancestors {
                self.tree.remove_node(ancestor);
            }
        }
    }
}

impl<T: Integer + Clone> SubAssign<SwissBox<T>> for SwissBox<T> {
    fn sub_assign(&mut self, rhs: SwissBox<T>) {
        *self -= &rhs;
    }
}

impl<'a, T: Integer + Clone> SubAssign<&'a SwissBox<T>> for SwissBox<T> {
    fn sub_assign(&mut self, rhs: &'a SwissBox<T>) {
        for node in rhs.tree.externals(Outgoing) {
            *self -= &rhs.tree[node];
        }
    }
}

impl<T: Integer + CheckedAdd + CheckedSub + CheckedMul> SwissBox<T> {
    pub fn volume(&self) -> Option<T> {
        let mut volume: T = T::zero();

        for node in self.tree.externals(Outgoing) {
            let r#box = &self.tree[node];
            volume = volume.checked_add(&r#box.volume()?)?;
        }

        Some(volume)
    }
}

#[cfg(test)]
mod tests {
    use super::{Box, SwissBox};

    #[test]
    fn empty_swiss_box_has_no_volume() {
        assert_eq!(SwissBox::<i32>::default().volume().unwrap(), 0);
        assert_eq!(SwissBox::new(Box::<i32>::default()).volume().unwrap(), 0);
    }

    #[test]
    fn swiss_box_has_volume_of_initial_box() {
        assert_eq!(
            SwissBox::new(Box {
                x: (0..3).into(),
                y: (0..3).into(),
                z: (0..3).into(),
            })
            .volume()
            .unwrap(),
            27
        );
    }

    #[test]
    fn rubik_center() {
        let mut cube = SwissBox::new(Box {
            x: (0..3).into(),
            y: (0..3).into(),
            z: (0..3).into(),
        });

        let center = Box {
            x: (1..2).into(),
            y: (1..2).into(),
            z: (1..2).into(),
        };

        cube -= center;

        assert_eq!(cube.volume().unwrap(), 27 - 1);
    }

    #[test]
    fn rubik_corner() {
        let mut cube = SwissBox::new(Box {
            x: (0..3).into(),
            y: (0..3).into(),
            z: (0..3).into(),
        });

        let corner = Box {
            x: (2..3).into(),
            y: (2..3).into(),
            z: (2..3).into(),
        };

        cube -= corner;

        assert_eq!(cube.volume().unwrap(), 27 - 1);
    }

    #[test]
    fn rubik_bar() {
        let mut cube = SwissBox::new(Box {
            x: (0..3).into(),
            y: (0..3).into(),
            z: (0..3).into(),
        });

        let bar = Box {
            x: (0..3).into(),
            y: (1..2).into(),
            z: (1..2).into(),
        };

        cube -= bar;

        assert_eq!(cube.volume().unwrap(), 27 - 3);
    }

    #[test]
    fn death_by_a_thousand_cuts() {
        let mut cube = SwissBox::new(Box {
            x: (0..3).into(),
            y: (0..3).into(),
            z: (0..3).into(),
        });

        for x in 0..3 {
            for y in 0..3 {
                for z in 0..3 {
                    cube -= Box {
                        x: (x..x + 1).into(),
                        y: (y..y + 1).into(),
                        z: (z..z + 1).into(),
                    };
                }
            }
        }

        assert_eq!(cube.volume().unwrap(), 0);
        assert!(cube.is_empty());
    }
}
