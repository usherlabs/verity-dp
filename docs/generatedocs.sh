#!/bin/bash
# run this command to generate docs `bash generatedocs.sh`

# Create an array of destinations we want to generate docs for
projects_dirs=(
    "../ic/libraries"
    "../rs/verity-client"
    "../rs/local-verify"
    "../rs/remote-verify"
    "../rs/verity-client"
    "../zk/guest"
    "../zk/host"
)

# The template for the list
item_html_template='
<li class="packages__list__item">
    <a target="_blank" href="package_path_placeholder">
        <span class="package__name"> name_placeholder </span>
        <span class="package__description"> description_placeholder </span>
    </a>
</li>
'

# The filled template
package_list_html=''

for current_dir in ${projects_dirs[@]}; do
    # Obtain the Cargo.toml file to the current_directory in the integration
    manifest_path="$current_dir/Cargo.toml"

    # Use grep and sed to extract the value of the name, description and version of the package
    package_name=$(grep -oP '^name\s*=\s*"\K[^"]+' "$manifest_path")
    package_description=$(grep -oP '^description\s*=\s*"\K[^"]+' "$manifest_path")
    package_version=$(grep -oP '^version\s*=\s*"\K[^"]+' "../Cargo.toml")

    package_name_and_version="$package_name-$package_version"
    underscore_package_name=${package_name//-/_}

    # Obtain the target_directory where the docs will be saved based on the package name
    target_dir="./packages/$package_name"

    # Run the command to generate the docs
    cargo doc --manifest-path "$manifest_path" --target-dir "$target_dir"

    path_to_docs=("./packages/$package_name/doc/$underscore_package_name/index.html")

    # Replace name and description placeholders in the template
    processed_html=$(echo "$item_html_template" | sed -e "s|name_placeholder|$package_name_and_version|g" -e "s|description_placeholder|$package_description|g" -e "s|package_path_placeholder|$path_to_docs|g")

    # Append the processed HTML to the final string
    package_list_html+="$processed_html"

done

input_file_path="./base.html"
output_file_path="./index.html"

input_file=$(cat $input_file_path)

# Split the content into parts using 'packages_list' as the delimiter
before_placeholder=${input_file%%packages_list*}
after_placeholder=${input_file#*packages_list}

full_html="$before_placeholder $package_list_html $after_placeholder"

# echo "Replaced 'packages_list' with the content of 'list_items' in $input_file_path."
echo $full_html >$output_file_path
