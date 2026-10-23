#[cfg(test)]
mod position_variants {
    use taffy::prelude::*;
    use taffy::{NodeId, Point, TaffyTree};

    fn leaf_with_inset(position: Position, display: Display) -> (TaffyTree<()>, NodeId) {
        let mut taffy: TaffyTree<()> = TaffyTree::new();
        let child = taffy
            .new_leaf(Style {
                position,
                size: Size { width: length(10.0), height: length(10.0) },
                inset: Rect { left: length(5.0), right: auto(), top: length(7.0), bottom: auto() },
                ..Default::default()
            })
            .unwrap();
        let root = taffy
            .new_with_children(
                Style { display, size: Size { width: length(100.0), height: length(100.0) }, ..Default::default() },
                &[child],
            )
            .unwrap();
        taffy.compute_layout(root, Size::MAX_CONTENT).unwrap();
        (taffy, child)
    }

    #[test]
    fn static_and_sticky_ignore_inset() {
        for display in [Display::Block, Display::Flex, Display::Grid] {
            for position in [Position::Static, Position::Sticky] {
                let (taffy, child) = leaf_with_inset(position, display);
                let layout = taffy.layout(child).unwrap();
                assert_eq!(layout.location, Point { x: 0.0, y: 0.0 }, "{position:?} in {display:?}");
            }
            let (taffy, child) = leaf_with_inset(Position::Relative, display);
            assert_eq!(taffy.layout(child).unwrap().location, Point { x: 5.0, y: 7.0 }, "Relative in {display:?}");
        }
    }

    /// Gecko halves an absolutely positioned box's free space in whole 1/60 px units,
    /// truncating toward zero: 63 units give the start margin 31 in both axes, -63 give -31.
    #[test]
    fn absolute_auto_margins_split_in_whole_app_units() {
        let mut taffy: TaffyTree<()> = TaffyTree::new();
        taffy.disable_rounding();
        let child = taffy
            .new_leaf(Style {
                position: Position::Absolute,
                size: Size { width: length(98.95), height: length(101.05) },
                inset: Rect { left: length(0.0), right: length(0.0), top: length(0.0), bottom: length(0.0) },
                margin: Rect { left: auto(), right: auto(), top: auto(), bottom: auto() },
                ..Default::default()
            })
            .unwrap();
        let root = taffy
            .new_with_children(
                Style { size: Size { width: length(100.0), height: length(100.0) }, ..Default::default() },
                &[child],
            )
            .unwrap();
        taffy.compute_layout(root, Size::MAX_CONTENT).unwrap();
        let layout = taffy.layout(child).unwrap();
        assert!((layout.location.x - 31.0 / 60.0).abs() < 1e-4, "x {}", layout.location.x);
        assert!((layout.location.y + 31.0 / 60.0).abs() < 1e-4, "y {}", layout.location.y);
    }

    #[test]
    fn sticky_is_positioned_but_in_flow() {
        assert!(Position::Sticky.is_positioned());
        assert!(!Position::Sticky.is_out_of_flow());
        assert!(!Position::Static.is_positioned());
    }
}
