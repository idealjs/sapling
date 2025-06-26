#!/bin/bash

# 创建结果输出文件
output_file="packages/sapling_shared/src/function_results.txt"
echo "Function Search Results" > $output_file
echo "======================" >> $output_file
echo "Generated at: $(date)" >> $output_file
echo "" >> $output_file

# 通用搜索函数
search_functions() {
    local category=$1
    local patterns=$2
    
    echo "## $category" >> $output_file
    echo "Searching $category..."
    echo "" >> $output_file
    
    find . -type f -name "*.rs" -exec grep -l "fn \($patterns\)" {} \; | while read -r file; do
        echo "Found in: $file" >> $output_file
        grep -h "fn \($patterns\)" "$file" | sed 's/^[[:space:]]*/    /' >> $output_file
    done
    echo "" >> $output_file
}

# 搜索不同类别的函数
search_functions "DOM Functions" "transform_attributes\|set_attr\|transform_children\|process_spreads\|transform_element\|detect_expressions\|create_placeholder\|find_last_element\|detect_resolvable_event_handler"

search_functions "Component Functions" "transform_component\|transform_component_children\|convert_component_identifier\|transform_fragment_children"

search_functions "Transform Functions" "transform_node\|transform_this\|transform_jsx\|get_create_template\|get_target_function_parent"

search_functions "Utility Functions Group 1" "is_dynamic\|escape_html\|transform_condition\|wrapped_by_text\|convert_jsx_identifier\|jsx_element_name_to_string"
search_functions "Utility Functions Group 2" "filter_children\|check_length\|trim_whitespace\|tag_name_to_identifier\|get_static_expression\|can_native_spread"
search_functions "Utility Functions Group 3" "get_numbered_id\|escape_string_for_template\|get_config\|get_renderer_config\|get_tag_name\|is_component"

search_functions "Validation Functions" "is_invalid_markup"

search_functions "SSR Functions" "transform_to_object\|transform_classlist_object\|escape_expression\|normalize_attributes"

search_functions "Universal Functions" "transform_attributes_universal\|process_spreads_universal\|transform_children_universal\|transform_element_universal"

echo "Search complete. Results have been saved to $output_file"
