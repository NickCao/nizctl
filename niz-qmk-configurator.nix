{ runCommand, dockerTools, makeWrapper
, gnutar
, xdg-utils
}:

let
  qmk-configurator-image = dockerTools.pullImage {
    imageName = "docker.io/qmkfm/qmk_configurator";
    imageDigest = "sha256:50e65561cc8b24049f77f4c79fc1e3bb01a7c2410cd351774ba8942c37b6e485";
    sha256 = "0j61sr0l1dpw7r5swbpvssqfl98qpxnfgmiss9icpzh2rr9r3n5c";
    finalImageName = "docker.io/qmkfm/qmk_configurator";
    finalImageTag = "latest";
  };

in
  runCommand "niz-qmk-configurator" {
    nativeBuildInputs = [ gnutar makeWrapper ];
    buildInputs = [ xdg-utils ];
  } ''
    mkdir -p "$out/opt" "$out/bin"

    mkdir image
    tar -xf ${qmk-configurator-image} -C image/

    # Find layer in docker image containing actual configurator code
    found_layer=
    for layer in image/*.tar; do
      if (tar -tf "$layer" 2>/dev/null || true) | grep --silent '^qmk_configurator/dist'; then
        found_layer="$layer"
      fi
    done


    if [ -z "$found_layer" ]; then
      echo No qmk_configurator/dist layer found >&2
      exit 1
    else
      echo "Found qmk_configurator/dist in $layer"
    fi

    # Extract that layer to $out/opt
    tar -xf "$found_layer" -C $out/opt

    # Replace keyboard info URL references
    sed -i "s/keyboards\.qmk\.fm/raw\.githubusercontent\.com\/NickCao\/nizctl\/master\/data/g" "$out"/opt/qmk_configurator/dist/js/*.js

    # Make a convenient runner
    cat <<END >"$out/bin/niz-qmk-configurator"
    #!/bin/sh
    exec xdg-open $out/opt/qmk_configurator/dist/index.html
    END

    chmod +x "$out/bin/niz-qmk-configurator"
  ''
