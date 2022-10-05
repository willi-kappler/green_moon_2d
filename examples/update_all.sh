for folder in text1 text2 text3 sprite1; do
  echo $folder
  cd $folder
  cargo clean
  cargo update
  cargo check
  cd ..
done
