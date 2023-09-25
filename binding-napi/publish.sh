TARGET=aarch64-apple-darwin

mkdir -p repo
cp index.js repo/index.js
cp index.d.ts repo/index.d.ts
cp package.json repo/package.json
mv *.node repo
cd repo
npm pack
echo $TARGET
git init
git remote add origin https://github.com/abcd-ts/rust-wasm-github-actions-test.git
echo "*.tgz binary" > .gitattribute
git add --all
git commit -m "commit package"
git config pull.rebase false
git pull -Xours --allow-unrelated-histories origin $TARGET
git push origin master:$TARGET
cd ..
rm -rf repo
