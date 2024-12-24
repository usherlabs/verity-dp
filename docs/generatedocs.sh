#!/bin/bash
# run this command to generate docs `bash generatedocs.sh`
# ! Make sure you run the script directly from the docs directory it resides in

main() {
    # Create an array of destinations we want to generate docs for
    projects_dirs=(
        "../ic/libraries"
        "../rs/verify-local"
        "../rs/verify-remote"
        "../rs/verify-tls"
        "../rs/verity-client"
        "../zk/guest"
        "../zk/host"
    )

    # The template for the list
    item_html_template=$(cat ./templates/item.hbs)

    # The filled template
    package_list_html=''
    package_version=$(grep '^version\s*=' "../Cargo.toml" | awk -F '"' '{print $2}')

    for current_dir in ${projects_dirs[@]}; do
        # Obtain the Cargo.toml file to the current_directory in the integration
        manifest_path="$current_dir/Cargo.toml"

        # Use grep and sed to extract the value of the name, description and version of the package
        # package_name=$(grep -oP '^name\s*=\s*"\K[^"]+' "$manifest_path")
        # package_description=$(grep -oP '^description\s*=\s*"\K[^"]+' "$manifest_path")
        package_name=$(grep '^name\s*=' "$manifest_path" | awk -F '"' '{print $2}')
        package_description=$(grep '^description\s*=' "$manifest_path" | awk -F '"' '{print $2}')

        package_name_and_version="$package_name-$package_version"

        # Replace all hyphens with underscores i/e pkg-1 => pkg_2
        underscore_package_name=${package_name//-/_}

        # Obtain the target_directory where the docs will be saved based on the package name
        target_dir="./packages/$package_name"

        # Run the command to generate the docs
        cargo doc --manifest-path "$manifest_path" --target-dir "$target_dir"
        path_to_docs=("./packages/$package_name/doc/$underscore_package_name/index.html")

        # Delete the build file because it takes space and is unnecessary to the docs
        path_to_build=("./packages/$package_name/debug")
        rm -rf $path_to_build

        # Replace name and description placeholders in the template
        processed_html=$(echo "$item_html_template" | sed -e "s|{{name_placeholder}}|$package_name_and_version|g" -e "s|{{description_placeholder}}|$package_description|g" -e "s|package_path_placeholder|$path_to_docs|g")

        # Append the processed HTML to the final string
        package_list_html+="$processed_html"
    done

    input_file_path="./templates/base.hbs"
    output_file_path="./index.html"

    # Read the input file into a variable
    input_file=$(cat $input_file_path)

    # Split the content into parts using '{{packages_list}}' as the delimiter
    before_placeholder=${input_file%%"{{packages_list}}"*}
    after_placeholder=${input_file#*"{{packages_list}}"}

    # Create the full html content by putting the html content in the middle of the htmltemplate
    # ! have to do this instead of a direct replace because the replace command fails due to special characters present in an html template
    full_html="$before_placeholder $package_list_html $after_placeholder"

    # Write the processed HTML to the output file path
    echo "$full_html" > $output_file_path
}

# Call the main function
main
