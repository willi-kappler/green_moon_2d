for folder in *; do
  if [[ -d $folder ]]; then
    echo $folder
    cd $folder
    cargo clean
    cargo update
    cargo check
    cd ..
  fi
done
