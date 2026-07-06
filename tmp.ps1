  $Repo = "LocalSandBox/local-sandbox"
  $SourceZip = "$env:USERPROFILE\Desktop\qemu.zip"
  $PackageRevision = "lsb1"

  $Work = Join-Path $env:TEMP "lsb-qemu-artifact-$([guid]::NewGuid())"
  $Expanded = Join-Path $Work "expanded"
  New-Item -ItemType Directory -Force -Path $Expanded | Out-Null

  Expand-Archive -LiteralPath $SourceZip -DestinationPath $Expanded

  # If zip has one top-level dir, use it; otherwise use expanded root.
  if (Test-Path (Join-Path $Expanded "qemu-system-x86_64.exe")) {
    $PayloadRoot = $Expanded
  } else {
    $TopDirs = @(Get-ChildItem $Expanded -Directory)
    if ($TopDirs.Count -eq 1 -and (Test-Path (Join-Path $TopDirs[0].FullName "qemu-system-x86_64.exe"))) {
      $PayloadRoot = $TopDirs[0].FullName
    } else {
      throw "Expected qemu.zip to contain qemu-system-x86_64.exe at root or under one top-level directory."
    }
  }

  $QemuExe = Join-Path $PayloadRoot "qemu-system-x86_64.exe"
  $QemuImg = Join-Path $PayloadRoot "qemu-img.exe"

  if (-not (Test-Path $QemuExe -PathType Leaf)) { throw "Missing qemu-system-x86_64.exe" }
  if (-not (Test-Path $QemuImg -PathType Leaf)) { throw "Missing qemu-img.exe" }

  $VersionLine = (& $QemuExe --version | Select-Object -First 1)
  if ($VersionLine -notmatch "version\s+([0-9]+(\.[0-9]+){1,2})") {
    throw "Could not parse QEMU version from: $VersionLine"
  }

  $QemuVersion = $Matches[1]
  $PackageVersion = "qemu-$QemuVersion-$PackageRevision"
  $PackageRoot = Join-Path $Work $PackageVersion

  New-Item -ItemType Directory -Force -Path $PackageRoot | Out-Null
  Copy-Item -LiteralPath (Join-Path $PayloadRoot "*") -Destination $PackageRoot -Recurse -Force

  $ManifestPath = Join-Path $PackageRoot "manifest.json"
  $Files = Get-ChildItem $PackageRoot -File -Recurse |
    Where-Object { $_.FullName -ne $ManifestPath } |
    ForEach-Object {
      [pscustomobject]@{
        path = $_.FullName.Substring($PackageRoot.Length + 1).Replace("\", "/")
        size_bytes = $_.Length
        sha256 = (Get-FileHash $_.FullName -Algorithm SHA256).Hash.ToLowerInvariant()
      }
    }

  [ordered]@{
    schema_version = 1
    package_version = $PackageVersion
    qemu_version = $QemuVersion
    platform = "windows-x86_64"
    qemu_system_x86_64 = "qemu-system-x86_64.exe"
    qemu_img = "qemu-img.exe"
    generated_at = (Get-Date).ToUniversalTime().ToString("o")
    files = $Files
  } | ConvertTo-Json -Depth 6 | Set-Content -LiteralPath $ManifestPath -Encoding UTF8

  $ReleaseTag = "qemu-windows-x86_64-v$QemuVersion-$PackageRevision"
  $ArtifactName = "lsb-qemu-windows-x86_64-qemu-$QemuVersion-$PackageRevision.tar.gz"
  $ArtifactPath = "$env:USERPROFILE\Desktop\$ArtifactName"

  tar -czf $ArtifactPath -C $Work $PackageVersion

  $Sha256 = (Get-FileHash $ArtifactPath -Algorithm SHA256).Hash.ToLowerInvariant()
  $ArtifactUrl = "https://github.com/$Repo/releases/download/$ReleaseTag/$ArtifactName"

  Write-Host "QEMU version: $QemuVersion"
  Write-Host "package revision: $PackageRevision"
  Write-Host "GitHub release tag: $ReleaseTag"
  Write-Host "artifact name: $ArtifactName"
  Write-Host "artifact URL: $ArtifactUrl"
  Write-Host "sha256: $Sha256"
