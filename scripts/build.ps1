param(
  [switch]$Install
)

$ErrorActionPreference = 'Stop'
$PSNativeCommandUseErrorActionPreference = $true

Get-ChildItem 'core/pkg' -Recurse | Remove-Item

wasm-pack build core --release

if ($Install) {
  pnpm install
}

pnpm run -F web build
