fn main() {
    glib_build_tools::compile_resources(
        "composite_templates/resources",
        "composite_templates/resources/resources.gresource.xml",
        "composite_templates.gresource",
    );
    glib_build_tools::compile_resources(
        "todo/resources",
        "todo/resources/resources.gresource.xml",
        "todo.gresource",
    );
}