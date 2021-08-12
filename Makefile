build:
	@wasm-pack build --target web --out-name app

package:
	@rsync -r pkg/snippets public/
	@rsync pkg/app.js public/
	@rsync pkg/app_bg.wasm public/

rollup:
	@([ -z "$(x)" ] && echo "not yet working as expected") \
	    || npx rollup --plugin wasm --format=iife --input pkg/app.js --file public/app.js --output.name app

serve:
	@npx tauri dev
