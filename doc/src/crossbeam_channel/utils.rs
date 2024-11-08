<!DOCTYPE html><html lang="en"><head><meta charset="utf-8"><meta name="viewport" content="width=device-width, initial-scale=1.0"><meta name="generator" content="rustdoc"><meta name="description" content="Source of the Rust file `/Users/jeyakumar/.cargo/registry/src/index.crates.io-6f17d22bba15001f/crossbeam-channel-0.5.12/src/utils.rs`."><title>utils.rs - source</title><script>if(window.location.protocol!=="file:")document.head.insertAdjacentHTML("beforeend","SourceSerif4-Regular-46f98efaafac5295.ttf.woff2,FiraSans-Regular-018c141bf0843ffd.woff2,FiraSans-Medium-8f9a781e4970d388.woff2,SourceCodePro-Regular-562dcc5011b6de7d.ttf.woff2,SourceCodePro-Semibold-d899c5a5c4aeb14a.ttf.woff2".split(",").map(f=>`<link rel="preload" as="font" type="font/woff2" crossorigin href="../../static.files/${f}">`).join(""))</script><link rel="stylesheet" href="../../static.files/normalize-76eba96aa4d2e634.css"><link rel="stylesheet" href="../../static.files/rustdoc-081576b923113409.css"><meta name="rustdoc-vars" data-root-path="../../" data-static-root-path="../../static.files/" data-current-crate="crossbeam_channel" data-themes="" data-resource-suffix="" data-rustdoc-version="1.79.0 (129f3b996 2024-06-10)" data-channel="1.79.0" data-search-js="search-bf21c90c8c1d92b1.js" data-settings-js="settings-4313503d2e1961c2.js" ><script src="../../static.files/storage-e32f0c247825364d.js"></script><script defer src="../../static.files/src-script-e66d777a5a92e9b2.js"></script><script defer src="../../src-files.js"></script><script defer src="../../static.files/main-20a3ad099b048cf2.js"></script><noscript><link rel="stylesheet" href="../../static.files/noscript-09095024cf37855e.css"></noscript><link rel="alternate icon" type="image/png" href="../../static.files/favicon-32x32-422f7d1d52889060.png"><link rel="icon" type="image/svg+xml" href="../../static.files/favicon-2c020d218678b618.svg"></head><body class="rustdoc src"><!--[if lte IE 11]><div class="warning">This old browser is unsupported and will most likely display funky things.</div><![endif]--><nav class="sidebar"><div class="src-sidebar-title"><h2>Files</h2></div></nav><div class="sidebar-resizer"></div><main><nav class="sub"><form class="search-form"><span></span><div id="sidebar-button" tabindex="-1"><a href="../../crossbeam_channel/all.html" title="show sidebar"></a></div><input class="search-input" name="search" aria-label="Run search in the documentation" autocomplete="off" spellcheck="false" placeholder="Type ‘S’ or ‘/’ to search, ‘?’ for more options…" type="search"><div id="help-button" tabindex="-1"><a href="../../help.html" title="help">?</a></div><div id="settings-menu" tabindex="-1"><a href="../../settings.html" title="settings">Settings</a></div></form></nav><section id="main-content" class="content"><div class="example-wrap"><div data-nosnippet><pre class="src-line-numbers"><a href="#1" id="1">1</a>
<a href="#2" id="2">2</a>
<a href="#3" id="3">3</a>
<a href="#4" id="4">4</a>
<a href="#5" id="5">5</a>
<a href="#6" id="6">6</a>
<a href="#7" id="7">7</a>
<a href="#8" id="8">8</a>
<a href="#9" id="9">9</a>
<a href="#10" id="10">10</a>
<a href="#11" id="11">11</a>
<a href="#12" id="12">12</a>
<a href="#13" id="13">13</a>
<a href="#14" id="14">14</a>
<a href="#15" id="15">15</a>
<a href="#16" id="16">16</a>
<a href="#17" id="17">17</a>
<a href="#18" id="18">18</a>
<a href="#19" id="19">19</a>
<a href="#20" id="20">20</a>
<a href="#21" id="21">21</a>
<a href="#22" id="22">22</a>
<a href="#23" id="23">23</a>
<a href="#24" id="24">24</a>
<a href="#25" id="25">25</a>
<a href="#26" id="26">26</a>
<a href="#27" id="27">27</a>
<a href="#28" id="28">28</a>
<a href="#29" id="29">29</a>
<a href="#30" id="30">30</a>
<a href="#31" id="31">31</a>
<a href="#32" id="32">32</a>
<a href="#33" id="33">33</a>
<a href="#34" id="34">34</a>
<a href="#35" id="35">35</a>
<a href="#36" id="36">36</a>
<a href="#37" id="37">37</a>
<a href="#38" id="38">38</a>
<a href="#39" id="39">39</a>
<a href="#40" id="40">40</a>
<a href="#41" id="41">41</a>
<a href="#42" id="42">42</a>
<a href="#43" id="43">43</a>
<a href="#44" id="44">44</a>
<a href="#45" id="45">45</a>
<a href="#46" id="46">46</a>
<a href="#47" id="47">47</a>
<a href="#48" id="48">48</a>
<a href="#49" id="49">49</a>
<a href="#50" id="50">50</a>
<a href="#51" id="51">51</a>
<a href="#52" id="52">52</a>
<a href="#53" id="53">53</a>
<a href="#54" id="54">54</a>
<a href="#55" id="55">55</a>
<a href="#56" id="56">56</a>
<a href="#57" id="57">57</a>
<a href="#58" id="58">58</a>
</pre></div><pre class="rust"><code><span class="doccomment">//! Miscellaneous utilities.

</span><span class="kw">use </span>std::cell::Cell;
<span class="kw">use </span>std::num::Wrapping;
<span class="kw">use </span>std::thread;
<span class="kw">use </span>std::time::{Duration, Instant};

<span class="doccomment">/// Randomly shuffles a slice.
</span><span class="kw">pub</span>(<span class="kw">crate</span>) <span class="kw">fn </span>shuffle&lt;T&gt;(v: <span class="kw-2">&amp;mut </span>[T]) {
    <span class="kw">let </span>len = v.len();
    <span class="kw">if </span>len &lt;= <span class="number">1 </span>{
        <span class="kw">return</span>;
    }

    <span class="macro">std::thread_local!</span> {
        <span class="kw">static </span>RNG: Cell&lt;Wrapping&lt;u32&gt;&gt; = <span class="kw">const </span>{ Cell::new(Wrapping(<span class="number">1_406_868_647</span>)) };
    }

    <span class="kw">let _ </span>= RNG.try_with(|rng| {
        <span class="kw">for </span>i <span class="kw">in </span><span class="number">1</span>..len {
            <span class="comment">// This is the 32-bit variant of Xorshift.
            //
            // Source: https://en.wikipedia.org/wiki/Xorshift
            </span><span class="kw">let </span><span class="kw-2">mut </span>x = rng.get();
            x ^= x &lt;&lt; <span class="number">13</span>;
            x ^= x &gt;&gt; <span class="number">17</span>;
            x ^= x &lt;&lt; <span class="number">5</span>;
            rng.set(x);

            <span class="kw">let </span>x = x.<span class="number">0</span>;
            <span class="kw">let </span>n = i + <span class="number">1</span>;

            <span class="comment">// This is a fast alternative to `let j = x % n`.
            //
            // Author: Daniel Lemire
            // Source: https://lemire.me/blog/2016/06/27/a-fast-alternative-to-the-modulo-reduction/
            </span><span class="kw">let </span>j = ((x <span class="kw">as </span>u64).wrapping_mul(n <span class="kw">as </span>u64) &gt;&gt; <span class="number">32</span>) <span class="kw">as </span>u32 <span class="kw">as </span>usize;

            v.swap(i, j);
        }
    });
}

<span class="doccomment">/// Sleeps until the deadline, or forever if the deadline isn't specified.
</span><span class="kw">pub</span>(<span class="kw">crate</span>) <span class="kw">fn </span>sleep_until(deadline: <span class="prelude-ty">Option</span>&lt;Instant&gt;) {
    <span class="kw">loop </span>{
        <span class="kw">match </span>deadline {
            <span class="prelude-val">None </span>=&gt; thread::sleep(Duration::from_secs(<span class="number">1000</span>)),
            <span class="prelude-val">Some</span>(d) =&gt; {
                <span class="kw">let </span>now = Instant::now();
                <span class="kw">if </span>now &gt;= d {
                    <span class="kw">break</span>;
                }
                thread::sleep(d - now);
            }
        }
    }
}
</code></pre></div></section></main></body></html>