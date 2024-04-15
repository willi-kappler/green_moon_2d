for folder in *; do
  if [[ -d $folder ]]; then
    echo $folder
    cd $folder
    cargo check
    cd ..
  fi
done
