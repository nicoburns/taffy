#!/bin/bash

# Create versions of the fixtures with:
#   - The template wrapper stripped
#   - The "test-root" id replaced with the name of the test
rm yoga_test_fixtures/*.html
(cd test_fixtures && fd -x /bin/bash -c "htmlq -f {} -- '#test-root' | sd test-root {.} > ../yoga_test_fixtures/{}")

# Remove 'x' prefix from disabled fixtures (as yoga may not want to disable the same fixtures as taffy)
DISABLED_FIXTURES=`cd yoga_test_fixtures && fd "^x"`
for file_name in $DISABLED_FIXTURES; do
  new_file_name=`echo -n $file_name | sd '^x(.*)' '$1'`
  mv yoga_test_fixtures/$file_name yoga_test_fixtures/$new_file_name
done

# Remove 'gap' prefix from gap fixtures (yoga just uses row_gap and column_gap)
DISABLED_FIXTURES=`cd yoga_test_fixtures && fd "^gap_"`
for file_name in $DISABLED_FIXTURES; do
  new_file_name=`echo -n $file_name | sd '^gap_(.*)' '$1'`
  mv yoga_test_fixtures/$file_name yoga_test_fixtures/$new_file_name
done

# Delete CSS Grid fixtures as yoga doesn't support CSS Grid yet
(cd yoga_test_fixtures && rm grid*.html)

# Delete rounding fixtures as rounding may differ between taffy and yoga
# (cd yoga_test_fixtures && rm rounding*.html)

# Delete margin_left and margin_right tests (yoga uses start and end in it's tests)
rm yoga_test_fixtures/margin_left.html yoga_test_fixtures/margin_right.html

YGAbsolutePositionTests=(
  absolute_layout_width_height_start_top
  absolute_layout_width_height_end_bottom
  absolute_layout_start_top_end_bottom
  absolute_layout_width_height_start_top_end_bottom
  do_not_clamp_height_of_absolute_node_to_height_of_its_overflow_hidden_parent
  absolute_layout_within_border
  absolute_layout_align_items_and_justify_content_center
  absolute_layout_align_items_and_justify_content_flex_end
  absolute_layout_justify_content_center
  absolute_layout_align_items_center
  absolute_layout_align_items_center_on_child_only
  absolute_layout_align_items_and_justify_content_center_and_top_position
  absolute_layout_align_items_and_justify_content_center_and_bottom_position
  absolute_layout_align_items_and_justify_content_center_and_left_position
  absolute_layout_align_items_and_justify_content_center_and_right_position
  position_root_with_rtl_should_position_withoutdirection
  absolute_layout_percentage_bottom_based_on_parent_height
  absolute_layout_in_wrap_reverse_column_container
  absolute_layout_in_wrap_reverse_row_container
  absolute_layout_in_wrap_reverse_column_container_flex_end
  absolute_layout_in_wrap_reverse_row_container_flex_end
  percent_absolute_position_infinite_height
  absolute_layout_percentage_height_based_on_padded_parent
  absolute_layout_percentage_height_based_on_padded_parent_and_align_items_center
)


YGAlignContentTests=(
  align_content_flex_start
  align_content_flex_start_without_height_on_children
  align_content_flex_start_with_flex
  align_content_flex_end
  align_content_stretch
  align_content_spacebetween
  align_content_spacearound
  align_content_stretch_row
  align_content_stretch_row_with_children
  align_content_stretch_row_with_flex
  align_content_stretch_row_with_flex_no_shrink
  align_content_stretch_row_with_margin
  align_content_stretch_row_with_padding
  align_content_stretch_row_with_single_row
  align_content_stretch_row_with_fixed_height
  align_content_stretch_row_with_max_height
  align_content_stretch_row_with_min_height
  align_content_stretch_column
  align_content_stretch_is_not_overriding_align_items
)

YGAlignItemsTests=(
  align_items_stretch
  align_items_center
  align_items_flex_start
  align_items_flex_end
  align_baseline
  align_baseline_child
  align_baseline_child_multiline
  align_baseline_child_multiline_override
  align_baseline_child_multiline_no_override_on_secondline
  align_baseline_child_top
  align_baseline_child_top2
  align_baseline_double_nested_child
  align_baseline_column
  align_baseline_child_margin
  align_baseline_child_padding
  align_baseline_multiline
  align_baseline_multiline_column
  align_baseline_multiline_column2
  align_baseline_multiline_row_and_column
  align_items_center_child_with_margin_bigger_than_parent
  align_items_flex_end_child_with_margin_bigger_than_parent
  align_items_center_child_without_margin_bigger_than_parent
  align_items_flex_end_child_without_margin_bigger_than_parent
  align_center_should_size_based_on_content
  align_stretch_should_size_based_on_parent
  align_flex_start_with_shrinking_children
  align_flex_start_with_stretching_children
  align_flex_start_with_shrinking_children_with_stretch
)

YGAlignSelfTests=(
  align_self_center
  align_self_flex_end
  align_self_flex_start
  align_self_flex_end_override_flex_start
  align_self_baseline
)

YGAndroidNewsFeedTests=(
  android_news_feed
)

YGBorderTests=(
  border_no_size
  border_container_match_child
  border_flex_child
  border_stretch_child
  border_center_child
)

YGDimensionTests=(
  wrap_child
  wrap_grandchild
)

YGDisplayTests=(
  display_none
  display_none_fixed_size
  display_none_with_margin
  display_none_with_child
  display_none_with_position
  display_none_with_position_absolute
)

YGFlexDirectionTests=(
  flex_direction_column_no_height
  flex_direction_row_no_width
  flex_direction_column
  flex_direction_row
  flex_direction_column_reverse
  flex_direction_row_reverse
)

YGFlexTests=(
  flex_basis_flex_grow_column
  flex_shrink_flex_grow_row
  flex_shrink_flex_grow_child_flex_shrink_other_child
  flex_basis_flex_grow_row
  flex_basis_flex_shrink_column
  flex_basis_flex_shrink_row
  flex_shrink_to_zero
  flex_basis_overrides_main_size
  flex_grow_shrink_at_most
  flex_grow_less_than_factor_one
)

YGFlexWrapTests=(
  wrap_column
  wrap_row
  wrap_row_align_items_flex_end
  wrap_row_align_items_center
  flex_wrap_children_with_min_main_overriding_flex_basis
  flex_wrap_wrap_to_child_height
  flex_wrap_align_stretch_fits_one_row
  wrap_reverse_row_align_content_flex_start
  wrap_reverse_row_align_content_center
  wrap_reverse_row_single_line_different_size
  wrap_reverse_row_align_content_stretch
  wrap_reverse_row_align_content_space_around
  wrap_reverse_column_fixed_size
  wrapped_row_within_align_items_center
  wrapped_row_within_align_items_flex_start
  wrapped_row_within_align_items_flex_end
  wrapped_column_max_height
  wrapped_column_max_height_flex
  wrap_nodes_with_content_sizing_overflowing_margin
  wrap_nodes_with_content_sizing_margin_cross
)


YGGapTests=(
  column_gap_flexible
  column_gap_inflexible
  column_gap_mixed_flexible
  column_gap_child_margins
  column_row_gap_wrapping
  column_gap_justify_flex_start
  column_gap_justify_center
  column_gap_justify_flex_end
  column_gap_justify_space_between
  column_gap_justify_space_around
  column_gap_justify_space_evenly
  column_gap_wrap_align_flex_start
  column_gap_wrap_align_center
  column_gap_wrap_align_flex_end
  column_gap_wrap_align_space_between
  column_gap_wrap_align_space_around
  column_gap_wrap_align_stretch
  column_gap_determines_parent_width
  row_gap_align_items_stretch
  row_gap_align_items_end
  row_gap_column_child_margins
  row_gap_row_wrap_child_margins
  row_gap_determines_parent_height
)

YGJustifyContentTests=(
  justify_content_row_flex_start
  justify_content_row_flex_end
  justify_content_row_center
  justify_content_row_space_between
  justify_content_row_space_around
  justify_content_column_flex_start
  justify_content_column_flex_end
  justify_content_column_center
  justify_content_column_space_between
  justify_content_column_space_around
  justify_content_row_min_width_and_margin
  justify_content_row_max_width_and_margin
  justify_content_column_min_height_and_margin
  justify_content_colunn_max_height_and_margin
  justify_content_column_space_evenly
  justify_content_row_space_evenly
  justify_content_min_width_with_padding_child_width_greater_than_parent
  justify_content_min_width_with_padding_child_width_lower_than_parent
)

YGMarginTests=(
  margin_start
  margin_top
  margin_end
  margin_bottom
  margin_and_flex_row
  margin_and_flex_column
  margin_and_stretch_row
  margin_and_stretch_column
  margin_with_sibling_row
  margin_with_sibling_column
  margin_auto_bottom
  margin_auto_top
  margin_auto_bottom_and_top
  margin_auto_bottom_and_top_justify_center
  margin_auto_mutiple_children_column
  margin_auto_mutiple_children_row
  margin_auto_left_and_right_column
  margin_auto_left_and_right
  margin_auto_start_and_end_column
  margin_auto_start_and_end
  margin_auto_left_and_right_column_and_center
  margin_auto_left
  margin_auto_right
  margin_auto_left_and_right_stretch
  margin_auto_top_and_bottom_stretch
  margin_should_not_be_part_of_max_height
  margin_should_not_be_part_of_max_width
  margin_auto_left_right_child_bigger_than_parent
  margin_auto_left_child_bigger_than_parent
  margin_fix_left_auto_right_child_bigger_than_parent
  margin_auto_left_fix_right_child_bigger_than_parent
  margin_auto_top_stretching_child
  margin_auto_left_stretching_child
)

YGMinMaxDimensionTests=(
  max_width
  max_height
  min_height
  min_width
  justify_content_min_max
  align_items_min_max
  justify_content_overflow_min_max
  flex_grow_to_min
  flex_grow_in_at_most_container
  flex_grow_child
  flex_grow_within_constrained_min_max_column
  flex_grow_within_max_width
  flex_grow_within_constrained_max_width
  flex_root_ignored
  flex_grow_root_minimized
  flex_grow_height_maximized
  flex_grow_within_constrained_min_row
  flex_grow_within_constrained_min_column
  flex_grow_within_constrained_max_row
  flex_grow_within_constrained_max_column
  child_min_max_width_flexing
  min_width_overrides_width
  max_width_overrides_width
  min_height_overrides_height
  max_height_overrides_height
  min_max_percent_no_width_height
)

YGPaddingTests=(
  padding_no_size
  padding_container_match_child
  padding_flex_child
  padding_stretch_child
  padding_center_child
  child_with_padding_align_end
)

YGPercentageTests=(
  percentage_width_height
  percentage_position_left_top
  percentage_position_bottom_right
  percentage_flex_basis
  percentage_flex_basis_cross
  percentage_flex_basis_cross_min_height
  percentage_flex_basis_main_max_height
  percentage_flex_basis_cross_max_height
  percentage_flex_basis_main_max_width
  percentage_flex_basis_cross_max_width
  percentage_flex_basis_main_min_width
  percentage_flex_basis_cross_min_width
  percentage_multiple_nested_with_padding_margin_and_percentage_values
  percentage_margin_should_calculate_based_only_on_width
  percentage_padding_should_calculate_based_only_on_width
  percentage_absolute_position
  percentage_width_height_undefined_parent_size
  percent_within_flex_grow
  percentage_container_in_wrapping_container
  percent_absolute_position
)

YGRoundingTests=(
  rounding_flex_basis_flex_grow_row_width_of_100
  rounding_flex_basis_flex_grow_row_prime_number_width
  rounding_flex_basis_flex_shrink_row
  rounding_flex_basis_overrides_main_size
  rounding_total_fractial
  rounding_total_fractial_nested
  rounding_fractial_input_1
  rounding_fractial_input_2
  rounding_fractial_input_3
  rounding_fractial_input_4
  rounding_inner_node_controversy_horizontal
  rounding_inner_node_controversy_vertical
  rounding_inner_node_controversy_combined
)

YGSizeOverflowTests=(
  nested_overflowing_child
  nested_overflowing_child_in_constraint_parent
  parent_wrap_child_size_overflowing_parent
)

#Every known test
ALL_YOGA_TESTS="${YGAlignContentTests[@]} ${YGBorderTests[@]} ${YGMarginTests[@]} ${YGPaddingTests[@]} ${YGMinMaxDimensionTests[@]} ${YGAlignItemsTests[@]} ${YGDimensionTests[@]} ${YGJustifyContentTests[@]} ${YGFlexDirectionTests[@]} ${YGAlignSelfTests[@]} ${YGRoundingTests[@]} ${YGPercentageTests[@]} ${YGGapTests[@]} ${YGDisplayTests[@]} ${YGFlexTests[@]} ${YGAndroidNewsFeedTests[@]} ${YGFlexWrapTests[@]} ${YGSizeOverflowTests[@]} ${YGAbsolutePositionTests[@]}"
ALL_YOGA_TESTS_WITH_SPACE=" $ALL_YOGA_TESTS "

# for str in $EVERY_KNOWN_TEST; do
#   echo "$str.html"
# done

# for str in $EVERY_KNOWN_TEST; do
#   echo "$str.html"
# done

ALL_TAFFY_TESTS=`ls yoga_test_fixtures | sd '.html' ''`

for test_name in $ALL_TAFFY_TESTS; do
  if [[ ! $ALL_YOGA_TESTS_WITH_SPACE =~ " ${test_name} " ]]; then
      # whatever you want to do when array doesn't contain value
      echo "$test_name.html"
  fi
done
