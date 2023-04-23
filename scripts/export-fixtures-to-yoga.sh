#!/bin/bash

# List yoga fixture ids (should be run in fixtures dir in yoga repo)
# fd ".html" -j 1 -x /bin/bash -c 'echo {} && grep -oE '"'"'id="[^"]*"'"'"' {} | sd "id=\"(.*)\"" '"'"'$1'"'"' && echo ""'

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

# Delete measure fixtures as yoga's test generator doesn't support this yet
# (cd yoga_test_fixtures && rm measure*.html)

# Delete CSS Grid fixtures as yoga doesn't support CSS Grid yet
(cd yoga_test_fixtures && rm grid*.html)

# Delete percentage gap fixtures as yoga doesn't support percentage gaps yet
(cd yoga_test_fixtures && fd "gap(.*)percent(.*).html" -x rm {})

# Delete rounding fixtures as rounding may differ between taffy and yoga
# (cd yoga_test_fixtures && rm rounding*.html)

# Delete margin_left and margin_right tests (yoga uses start and end in it's tests)
rm yoga_test_fixtures/margin_left.html yoga_test_fixtures/margin_right.html

YGAbsolutePositionTests=(
  absolute_layout_width_height_start_top
  absolute_layout_width_height_end_bottom
  absolute_layout_row_width_height_end_bottom
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
  absolute_layout_percentage_height
  absolute_child_with_cross_margin
  absolute_child_with_main_margin
  absolute_child_with_max_height
  absolute_child_with_max_height_larger_shrinkable_grandchild

  # New from Taffy
  absolute_layout_child_order
  absolute_layout_no_size
  absolute_margin_bottom_left
  absolute_minmax_bottom_right_max
  absolute_minmax_bottom_right_min_max
  absolute_minmax_bottom_right_min_max_preferred
  absolute_minmax_top_left_bottom_right_max
  absolute_minmax_top_left_bottom_right_min_max
  absolute_padding_border_overrides_max_size
  absolute_padding_border_overrides_size
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

  # New from Taffy
  align_content_not_stretch_with_align_items_stretch
  align_content_space_around_single_line
  align_content_space_around_wrapped
  align_content_space_between_single_line
  align_content_space_between_wrapped
  align_content_space_evenly_single_line
  align_content_space_evenly_wrapped
)

YGAlignItemsTests=(
  align_items_stretch
  align_items_stretch_min_cross
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
  align_items_center_justify_content_center
  align_baseline_child_margin_percent
  align_baseline_nested_column

  # New from Taffy
  align_baseline_nested_child
  align_items_center_min_max_with_padding
  align_items_center_with_child_margin
  align_items_center_with_child_top
)

YGAlignSelfTests=(
  align_self_center
  align_self_center_undefined_max_height
  align_self_flex_end
  align_self_flex_start
  align_self_flex_end_override_flex_start
  align_self_baseline
)

YGAndroidNewsFeedTests=(
  android_news_feed
)

# New from Taffy
YGAspectRatioTests=(
  aspect_ratio_flex_column_fill_height
  aspect_ratio_flex_column_fill_max_height
  aspect_ratio_flex_column_fill_max_width
  aspect_ratio_flex_column_fill_min_height
  aspect_ratio_flex_column_fill_min_width
  aspect_ratio_flex_column_fill_width
  aspect_ratio_flex_column_fill_width_flex
  aspect_ratio_flex_column_stretch_fill_height
  aspect_ratio_flex_column_stretch_fill_max_height
  aspect_ratio_flex_column_stretch_fill_max_width
  aspect_ratio_flex_column_stretch_fill_min_height
  aspect_ratio_flex_column_stretch_fill_min_width
  aspect_ratio_flex_column_stretch_fill_width
  aspect_ratio_flex_row_fill_height
  aspect_ratio_flex_row_fill_max_height
  aspect_ratio_flex_row_fill_max_width
  aspect_ratio_flex_row_fill_min_height
  aspect_ratio_flex_row_fill_min_width
  aspect_ratio_flex_row_fill_width
  aspect_ratio_flex_row_fill_width_flex
  aspect_ratio_flex_row_stretch_fill_height
  aspect_ratio_flex_row_stretch_fill_max_height
  aspect_ratio_flex_row_stretch_fill_max_width
  aspect_ratio_flex_row_stretch_fill_min_height
  aspect_ratio_flex_row_stretch_fill_min_width
  aspect_ratio_flex_row_stretch_fill_width
  absolute_aspect_ratio_aspect_ratio_overrides_height_of_full_inset
  absolute_aspect_ratio_fill_height
  absolute_aspect_ratio_fill_height_from_inset
  absolute_aspect_ratio_fill_max_height
  absolute_aspect_ratio_fill_max_width
  absolute_aspect_ratio_fill_min_height
  absolute_aspect_ratio_fill_min_width
  absolute_aspect_ratio_fill_width
  absolute_aspect_ratio_fill_width_from_inset
  absolute_aspect_ratio_height_overrides_inset
  absolute_aspect_ratio_width_overrides_inset
)

YGBorderTests=(
  border_no_size
  border_container_match_child
  border_flex_child
  border_stretch_child
  border_center_child

  # New from Taffy
  border_no_child
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
  display_none_absolute_child

  # New from Taffy
  display_none_only_node
)

YGFlexDirectionTests=(
  flex_direction_column_no_height
  flex_direction_row_no_width
  flex_direction_column
  flex_direction_row
  flex_direction_column_reverse
  flex_direction_row_reverse
  flex_direction_column_reverse_no_height
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
  single_flex_child_after_absolute_child
  flex_basis_zero_undefined_main_size
  only_shrinkable_item_with_flex_basis_zero

  # New from Taffy
  container_with_unsized_child
  flex_basis_and_main_dimen_set_when_flexing
  flex_basis_larger_than_content_column
  flex_basis_larger_than_content_row
  flex_basis_slightly_smaller_then_content_with_flex_grow_large_size
  flex_basis_smaller_than_content_column
  flex_basis_smaller_than_content_row
  flex_basis_smaller_than_main_dimen_column
  flex_basis_smaller_than_main_dimen_row
  flex_basis_smaller_then_content_with_flex_grow_large_size
  flex_basis_smaller_then_content_with_flex_grow_small_size
  flex_basis_smaller_then_content_with_flex_grow_unconstraint_size
  flex_basis_smaller_then_content_with_flex_grow_very_large_size
  flex_basis_unconstraint_column
  flex_basis_unconstraint_row
  flex_basis_zero_undefined_main_size_hidden
  flex_column_relative_all_sides
  flex_grow_flex_basis_percent_min_max
  flex_row_relative_all_sides
  flex_shrink_by_outer_margin_with_max_size
  width_smaller_then_content_with_flex_grow_large_size
  width_smaller_then_content_with_flex_grow_small_size
  width_smaller_then_content_with_flex_grow_unconstraint_size
  width_smaller_then_content_with_flex_grow_very_large_size
  relative_position_should_not_nudge_siblings
  size_defined_by_child
  size_defined_by_child_with_border
  size_defined_by_child_with_padding
  size_defined_by_grand_child
  simple_child
  intrinsic_sizing_cross_size_column
  intrinsic_sizing_main_size_column
  intrinsic_sizing_main_size_column_nested
  intrinsic_sizing_main_size_column_wrap
  intrinsic_sizing_main_size_row
  intrinsic_sizing_main_size_row_nested
  intrinsic_sizing_main_size_row_wrap
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

  # New from Taffy
  wrap_reverse_column
  wrap_reverse_row
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

  # New from Taffy
  column_gap_flexible_undefined_parent
  column_gap_inflexible_undefined_parent
  column_gap_row_gap_wrapping
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

  # New from Taffy
  justify_content_column_max_height_and_margin
  justify_content_column_min_height_and_margin_bottom
  justify_content_column_min_height_and_margin_top
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

# New from Taffy
YGMeasureTests=(
  measure_child
  measure_child_absolute
  measure_child_constraint
  measure_child_constraint_padding_parent
  measure_child_with_flex_grow
  measure_child_with_flex_shrink
  measure_child_with_flex_shrink_hidden
  measure_child_with_min_size_greater_than_available_space
  measure_flex_basis_overrides_measure
  measure_height_overrides_measure
  measure_remeasure_child_after_growing
  measure_remeasure_child_after_shrinking
  measure_remeasure_child_after_stretching
  measure_root
  measure_stretch_overrides_measure
  measure_width_overrides_measure
)

YGMinMaxDimensionTests=(
  max_width
  min_width_larger_than_width
  min_height_larger_than_height
  max_height
  min_height_with_nested_fixed_height
  min_height
  min_width
  justify_content_min_max
  align_items_min_max
  align_items_center_min_max_with_padding
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
  min_max_percent_different_width_height
  undefined_height_with_min_max
  undefined_width_with_min_max
  undefined_width_with_min_max_row

  # New from Taffy
  min_height_overrides_height_on_root
  min_height_overrides_max_height
  min_width_overrides_max_width
  min_width_overrides_width_on_root
  max_height_overrides_height_on_root
  max_width_overrides_width_on_root
  padding_border_overrides_max_size
  padding_border_overrides_min_size
  padding_border_overrides_size
  padding_border_overrides_size_flex_basis_0
  padding_border_overrides_size_flex_basis_0_growable
  leaf_padding_border_overrides_max_size
  leaf_padding_border_overrides_min_size
  leaf_padding_border_overrides_size
)

YGPaddingTests=(
  padding_no_size
  padding_container_match_child
  padding_flex_child
  padding_stretch_child
  padding_center_child
  child_with_padding_align_end

  # New from Taffy
  padding_align_end_child
  padding_no_child
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
  percentage_main_max_height
  percentage_multiple_nested_with_padding_margin_and_percentage_values
  percentage_margin_should_calculate_based_only_on_width
  percentage_padding_should_calculate_based_only_on_width
  percentage_absolute_position
  percentage_width_height_undefined_parent_size
  percent_within_flex_grow
  percentage_container_in_wrapping_container
  percent_absolute_position
  percentage_different_width_height
  percentage_different_width_height_column

  # New from Taffy
  percentage_different_width_height
  percentage_different_width_height_column
  percentage_main_max_height
  percentage_moderate_complexity
  percentage_moderate_complexity2
  percentage_size_based_on_parent_inner_size
  percentage_size_of_flex_basis
  percentage_sizes_should_not_prevent_flex_shrinking
)

# New from Taffy
YGRegressionTests=(
  bevy_issue_7976_3_level
  bevy_issue_7976_4_level
  bevy_issue_7976_reduced
  bevy_issue_8017
  bevy_issue_8017_reduced
  bevy_issue_8082
  bevy_issue_8082_percent
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

  # New from Taffy
  rounding_fractial_input_5
  rounding_fractial_input_6
  rounding_fractial_input_7
)

YGSizeOverflowTests=(
  nested_overflowing_child
  nested_overflowing_child_in_constraint_parent
  parent_wrap_child_size_overflowing_parent

  # New from Taffy
  overflow_cross_axis
  overflow_main_axis
  overflow_main_axis_shrink_hidden
  overflow_main_axis_shrink_visible
)

#Every known test
ALL_YOGA_TESTS="${YGAlignContentTests[@]} ${YGBorderTests[@]} ${YGMarginTests[@]} ${YGMeasureTests[@]} ${YGRegressionTests[@]} ${YGPaddingTests[@]} ${YGMinMaxDimensionTests[@]} ${YGAlignItemsTests[@]} ${YGDimensionTests[@]} ${YGJustifyContentTests[@]} ${YGFlexDirectionTests[@]} ${YGAlignSelfTests[@]} ${YGAspectRatioTests[@]} ${YGRoundingTests[@]} ${YGPercentageTests[@]} ${YGGapTests[@]} ${YGDisplayTests[@]} ${YGFlexTests[@]} ${YGAndroidNewsFeedTests[@]} ${YGFlexWrapTests[@]} ${YGSizeOverflowTests[@]} ${YGAbsolutePositionTests[@]}"
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
      echo "$test_name" #.html"
  fi
done

TEST_CATEGORIES="YGAbsolutePositionTests YGAlignContentTests YGAlignItemsTests YGAlignSelfTests YGAndroidNewsFeedTests YGAspectRatioTests YGBorderTests YGDimensionTests YGDisplayTests YGFlexDirectionTests YGFlexTests YGFlexWrapTests YGGapTests YGJustifyContentTests YGMarginTests YGMeasureTests YGMinMaxDimensionTests YGPaddingTests YGPercentageTests YGRegressionTests YGRoundingTests YGSizeOverflowTests"

mkdir -p yoga_test_fixtures_grouped
rm yoga_test_fixtures_grouped/*.html
for cat_name in $TEST_CATEGORIES; do
  # Get list from list name
  eval TEST_LIST=\${$cat_name[@]}

  # Append .html to each fixture name
  # TEST_LIST_WITH_EXT=()
  for test in $TEST_LIST; do
     cat "yoga_test_fixtures/$test.html" >> yoga_test_fixtures_grouped/$cat_name.html
     echo "" >> yoga_test_fixtures_grouped/$cat_name.html
     echo "" >> yoga_test_fixtures_grouped/$cat_name.html
  done

  # echo $cat_name;
  # echo $test_list;
  # cat $test_list | 
done
