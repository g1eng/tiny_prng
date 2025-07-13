import {Pcg, Mt64, Xorshift64} from "tiny-prng-wasm";

const total_prng_steps = 10 ** 8;

let el = document.getElementById("demo");
el.innerHTML = `<h2>Online benchmarking for ${total_prng_steps / 1000000}M times instructions</h2>`

let now = new Date();
let before = (now.getSeconds()) * 1000 + now.getMilliseconds()
now = new Date();
let after = (now.getSeconds()) * 1000 + now.getMilliseconds()
let blank_diff = after - before

let a, b, c;
let [diff1, diff2, diff3] = [-1, -1, -1];

function generate_with_pcg() {
    let g = new Pcg(now.getMilliseconds())
    now = new Date();
    before = (now.getSeconds()) * 1000 + now.getMilliseconds()
    a = g.generate_list(total_prng_steps)
    now = new Date();
    after = (now.getSeconds()) * 1000 + now.getMilliseconds()
    diff1 = after - before - blank_diff
}

while (diff1 < 0) {
    generate_with_pcg()
}


function generate_with_mt64() {
    let g = new Mt64(now.getMilliseconds())
    now = new Date();
    before = (now.getSeconds()) * 1000 + now.getMilliseconds()
    b = g.generate_list(total_prng_steps)
    now = new Date();
    after = (now.getSeconds()) * 1000 + now.getMilliseconds()
    diff2 = after - before - blank_diff
}

while (diff2 < 0) {
    generate_with_mt64()
}

function generate_with_xorshift() {
    let g = new Xorshift64(now.getMilliseconds())
    now = new Date();
    before = (now.getSeconds()) * 1000 + now.getMilliseconds()
    c = g.generate_list(total_prng_steps)
    now = new Date();
    after = (now.getSeconds()) * 1000 + now.getMilliseconds()
    diff3 = after - before - blank_diff
}

while (diff3 < 0) {
    generate_with_xorshift()
}

el.innerHTML += `
<table>
<thead><td class="col-family">family</td><td>mode</td><td>time (msec)</td><td>data</td></thead>
<tbody>
<tr> <td class="col-family">PCG</td>              <td>PCG-XSL-RR-128/64</td> <td>${diff1}</td> <td>${a[0]} ${a[1]} ${a[2]}</td></tr>
<tr> <td class="col-family">Mersenne Twister</td> <td>MT19937_64</td>        <td>${diff2}</td> <td>${b[0]} ${b[1]} ${b[2]}</td></tr>
<tr> <td class="col-family">Xorshift</td>         <td>Xorshift64</td>        <td>${diff3}</td> <td>${c[0]} ${c[1]} ${c[2]}</td></tr>
</tbody>
</table>`;

