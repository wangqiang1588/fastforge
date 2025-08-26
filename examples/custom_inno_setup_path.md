# Example: Using Custom Inno Setup Path

This example demonstrates how to configure fastforge to use a custom Inno Setup installation path.

## Configuration

Add the following to your `windows/packaging/exe/make_config.yaml`:

```yaml
# Basic app information
app_id: 5B599538-42B1-4826-A479-AF079F21A65D
publisher: LeanFlutter
publisher_url: https://github.com/fastforgedev/fastforge
display_name: Hello 世界
create_desktop_icon: true

# Custom Inno Setup path
inno_setup_path: "D:\Tools\Inno Setup 6"

locales:
  - en
  - zh
```

## Use Cases

This feature is useful when:

1. **Non-standard Installation**: You installed Inno Setup in a custom directory
2. **Portable Installation**: You're using a portable version of Inno Setup
3. **Multiple Versions**: You have multiple versions of Inno Setup and want to use a specific one
4. **CI/CD Environments**: You have Inno Setup installed in a specific location in your build agents

## Fallback Behavior

If `inno_setup_path` is not specified or is `null`, fastforge will use the default path:
`C:\Program Files (x86)\Inno Setup 6`

This ensures backward compatibility with existing configurations.