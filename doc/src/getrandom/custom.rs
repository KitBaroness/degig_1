<!DOCTYPE html><html lang="en"><head><meta charset="utf-8"><meta name="viewport" content="width=device-width, initial-scale=1.0"><meta name="generator" content="rustdoc"><meta name="description" content="Source of the Rust file `/Users/jeyakumar/.cargo/registry/src/index.crates.io-6f17d22bba15001f/getrandom-0.2.14/src/custom.rs`."><title>custom.rs - source</title><script>if(window.location.protocol!=="file:")document.head.insertAdjacentHTML("beforeend","SourceSerif4-Regular-46f98efaafac5295.ttf.woff2,FiraSans-Regular-018c141bf0843ffd.woff2,FiraSans-Medium-8f9a781e4970d388.woff2,SourceCodePro-Regular-562dcc5011b6de7d.ttf.woff2,SourceCodePro-Semibold-d899c5a5c4aeb14a.ttf.woff2".split(",").map(f=>`<link rel="preload" as="font" type="font/woff2" crossorigin href="../../static.files/${f}">`).join(""))</script><link rel="stylesheet" href="../../static.files/normalize-76eba96aa4d2e634.css"><link rel="stylesheet" href="../../static.files/rustdoc-081576b923113409.css"><meta name="rustdoc-vars" data-root-path="../../" data-static-root-path="../../static.files/" data-current-crate="getrandom" data-themes="" data-resource-suffix="" data-rustdoc-version="1.79.0 (129f3b996 2024-06-10)" data-channel="1.79.0" data-search-js="search-bf21c90c8c1d92b1.js" data-settings-js="settings-4313503d2e1961c2.js" ><script src="../../static.files/storage-e32f0c247825364d.js"></script><script defer src="../../static.files/src-script-e66d777a5a92e9b2.js"></script><script defer src="../../src-files.js"></script><script defer src="../../static.files/main-20a3ad099b048cf2.js"></script><noscript><link rel="stylesheet" href="../../static.files/noscript-09095024cf37855e.css"></noscript><link rel="icon" href="https://www.rust-lang.org/favicon.ico"></head><body class="rustdoc src"><!--[if lte IE 11]><div class="warning">This old browser is unsupported and will most likely display funky things.</div><![endif]--><nav class="sidebar"><div class="src-sidebar-title"><h2>Files</h2></div></nav><div class="sidebar-resizer"></div><main><nav class="sub"><form class="search-form"><span></span><div id="sidebar-button" tabindex="-1"><a href="../../getrandom/all.html" title="show sidebar"></a></div><input class="search-input" name="search" aria-label="Run search in the documentation" autocomplete="off" spellcheck="false" placeholder="Type ‘S’ or ‘/’ to search, ‘?’ for more options…" type="search"><div id="help-button" tabindex="-1"><a href="../../help.html" title="help">?</a></div><div id="settings-menu" tabindex="-1"><a href="../../settings.html" title="settings">Settings</a></div></form></nav><section id="main-content" class="content"><div class="example-wrap"><div data-nosnippet><pre class="src-line-numbers"><a href="#1" id="1">1</a>
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
<a href="#59" id="59">59</a>
<a href="#60" id="60">60</a>
<a href="#61" id="61">61</a>
<a href="#62" id="62">62</a>
<a href="#63" id="63">63</a>
<a href="#64" id="64">64</a>
<a href="#65" id="65">65</a>
<a href="#66" id="66">66</a>
<a href="#67" id="67">67</a>
<a href="#68" id="68">68</a>
<a href="#69" id="69">69</a>
<a href="#70" id="70">70</a>
<a href="#71" id="71">71</a>
<a href="#72" id="72">72</a>
<a href="#73" id="73">73</a>
<a href="#74" id="74">74</a>
<a href="#75" id="75">75</a>
<a href="#76" id="76">76</a>
<a href="#77" id="77">77</a>
<a href="#78" id="78">78</a>
<a href="#79" id="79">79</a>
<a href="#80" id="80">80</a>
<a href="#81" id="81">81</a>
<a href="#82" id="82">82</a>
<a href="#83" id="83">83</a>
<a href="#84" id="84">84</a>
<a href="#85" id="85">85</a>
<a href="#86" id="86">86</a>
<a href="#87" id="87">87</a>
<a href="#88" id="88">88</a>
<a href="#89" id="89">89</a>
<a href="#90" id="90">90</a>
<a href="#91" id="91">91</a>
<a href="#92" id="92">92</a>
<a href="#93" id="93">93</a>
<a href="#94" id="94">94</a>
<a href="#95" id="95">95</a>
<a href="#96" id="96">96</a>
<a href="#97" id="97">97</a>
<a href="#98" id="98">98</a>
<a href="#99" id="99">99</a>
<a href="#100" id="100">100</a>
<a href="#101" id="101">101</a>
<a href="#102" id="102">102</a>
<a href="#103" id="103">103</a>
<a href="#104" id="104">104</a>
<a href="#105" id="105">105</a>
</pre></div><pre class="rust"><code><span class="doccomment">//! An implementation which calls out to an externally defined function.
</span><span class="kw">use crate</span>::{util::uninit_slice_fill_zero, Error};
<span class="kw">use </span>core::{mem::MaybeUninit, num::NonZeroU32};

<span class="doccomment">/// Register a function to be invoked by `getrandom` on unsupported targets.
///
/// ## Writing a custom `getrandom` implementation
///
/// The function to register must have the same signature as
/// [`getrandom::getrandom`](crate::getrandom). The function can be defined
/// wherever you want, either in root crate or a dependent crate.
///
/// For example, if we wanted a `failure-getrandom` crate containing an
/// implementation that always fails, we would first depend on `getrandom`
/// (for the [`Error`] type) in `failure-getrandom/Cargo.toml`:
/// ```toml
/// [dependencies]
/// getrandom = "0.2"
/// ```
/// Note that the crate containing this function does **not** need to enable the
/// `"custom"` Cargo feature.
///
/// Next, in `failure-getrandom/src/lib.rs`, we define our function:
/// ```rust
/// use core::num::NonZeroU32;
/// use getrandom::Error;
///
/// // Some application-specific error code
/// const MY_CUSTOM_ERROR_CODE: u32 = Error::CUSTOM_START + 42;
/// pub fn always_fail(buf: &amp;mut [u8]) -&gt; Result&lt;(), Error&gt; {
///     let code = NonZeroU32::new(MY_CUSTOM_ERROR_CODE).unwrap();
///     Err(Error::from(code))
/// }
/// ```
///
/// ## Registering a custom `getrandom` implementation
///
/// Functions can only be registered in the root binary crate. Attempting to
/// register a function in a non-root crate will result in a linker error.
/// This is similar to
/// [`#[panic_handler]`](https://doc.rust-lang.org/nomicon/panic-handler.html) or
/// [`#[global_allocator]`](https://doc.rust-lang.org/edition-guide/rust-2018/platform-and-target-support/global-allocators),
/// where helper crates define handlers/allocators but only the binary crate
/// actually _uses_ the functionality.
///
/// To register the function, we first depend on `failure-getrandom` _and_
/// `getrandom` in `Cargo.toml`:
/// ```toml
/// [dependencies]
/// failure-getrandom = "0.1"
/// getrandom = { version = "0.2", features = ["custom"] }
/// ```
///
/// Then, we register the function in `src/main.rs`:
/// ```rust
/// # mod failure_getrandom { pub fn always_fail(_: &amp;mut [u8]) -&gt; Result&lt;(), getrandom::Error&gt; { unimplemented!() } }
/// use failure_getrandom::always_fail;
/// use getrandom::register_custom_getrandom;
///
/// register_custom_getrandom!(always_fail);
/// ```
///
/// Now any user of `getrandom` (direct or indirect) on this target will use the
/// registered function. As noted in the
/// [top-level documentation](index.html#custom-implementations) this
/// registration only has an effect on unsupported targets.
</span><span class="attr">#[macro_export]
</span><span class="macro">macro_rules!</span> register_custom_getrandom {
    (<span class="macro-nonterminal">$path</span>:path) =&gt; {
        <span class="comment">// TODO(MSRV 1.37): change to unnamed block
        </span><span class="kw">const </span>__GETRANDOM_INTERNAL: () = {
            <span class="comment">// We use Rust ABI to be safe against potential panics in the passed function.
            </span><span class="attr">#[no_mangle]
            </span><span class="kw">unsafe fn </span>__getrandom_custom(dest: <span class="kw-2">*mut </span>u8, len: usize) -&gt; u32 {
                <span class="comment">// Make sure the passed function has the type of getrandom::getrandom
                </span><span class="kw">type </span>F = <span class="kw">fn</span>(<span class="kw-2">&amp;mut </span>[u8]) -&gt; ::core::result::Result&lt;(), <span class="macro-nonterminal">$crate::Error</span>&gt;;
                <span class="kw">let _</span>: F = <span class="macro-nonterminal">$crate::getrandom</span>;
                <span class="kw">let </span>f: F = <span class="macro-nonterminal">$path</span>;
                <span class="kw">let </span>slice = ::core::slice::from_raw_parts_mut(dest, len);
                <span class="kw">match </span>f(slice) {
                    <span class="prelude-val">Ok</span>(()) =&gt; <span class="number">0</span>,
                    <span class="prelude-val">Err</span>(e) =&gt; e.code().get(),
                }
            }
        };
    };
}

<span class="attr">#[allow(dead_code)]
</span><span class="kw">pub fn </span>getrandom_inner(dest: <span class="kw-2">&amp;mut </span>[MaybeUninit&lt;u8&gt;]) -&gt; <span class="prelude-ty">Result</span>&lt;(), Error&gt; {
    <span class="kw">extern </span><span class="string">"Rust" </span>{
        <span class="kw">fn </span>__getrandom_custom(dest: <span class="kw-2">*mut </span>u8, len: usize) -&gt; u32;
    }
    <span class="comment">// Previously we always passed a valid, initialized slice to
    // `__getrandom_custom`. Ensure `dest` has been initialized for backward
    // compatibility with implementations that rely on that (e.g. Rust
    // implementations that construct a `&amp;mut [u8]` slice from `dest` and
    // `len`).
    </span><span class="kw">let </span>dest = uninit_slice_fill_zero(dest);
    <span class="kw">let </span>ret = <span class="kw">unsafe </span>{ __getrandom_custom(dest.as_mut_ptr(), dest.len()) };
    <span class="kw">match </span>NonZeroU32::new(ret) {
        <span class="prelude-val">None </span>=&gt; <span class="prelude-val">Ok</span>(()),
        <span class="prelude-val">Some</span>(code) =&gt; <span class="prelude-val">Err</span>(Error::from(code)),
    }
}
</code></pre></div></section></main></body></html>