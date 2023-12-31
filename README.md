<h1><em>status: miserable failure</em></h1>
<h1>Ludum Dare 54</h1>
<p>To compile, run:</p>
<pre>cargo build --target wasm32-unknown-unknown</pre>
<p>This builds the WASM blob at <code>target/wasm32-unknown-unknown/debug/ld54.wasm</code>, which is already symlinked to <code>dist/game.wasm</code>. (Note: This is the <em>debug</em> build; for final release, <code>game.wasm</code> should be copied from <code>target/wasm32-unknown-unknown/<strong>release</strong>/ld54.wasm</code> instead.)</p>
<p><code>dist/mq_js_bundle.js</code> has been copied wholesale from <code><a href="https://github.com/not-fl3/macroquad/blob/1158aaf5c4f9fc89a91edd212ef6dfc7777e1395/js/mq_js_bundle.js">https://github.com/not-fl3/macroquad/blob/master/js/mq_js_bundle.js</a></code>, which has been made available under <a href="https://github.com/not-fl3/macroquad/blob/master/LICENSE-MIT">the MIT license</a> or <a href="https://github.com/not-fl3/macroquad/blob/master/LICENSE-APACHE">the Apache license</a> by <a href="https://github.com/not-fl3">Fedor Logachev</a>.</p>

<p>To serve locally:</p>
<pre>cd dist; python -m http.server</pre>
<p>(or any HTTP server of your choice)</p>
