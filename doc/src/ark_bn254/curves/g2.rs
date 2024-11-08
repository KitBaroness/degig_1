<!DOCTYPE html><html lang="en"><head><meta charset="utf-8"><meta name="viewport" content="width=device-width, initial-scale=1.0"><meta name="generator" content="rustdoc"><meta name="description" content="Source of the Rust file `/Users/jeyakumar/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ark-bn254-0.4.0/src/curves/g2.rs`."><title>g2.rs - source</title><script>if(window.location.protocol!=="file:")document.head.insertAdjacentHTML("beforeend","SourceSerif4-Regular-46f98efaafac5295.ttf.woff2,FiraSans-Regular-018c141bf0843ffd.woff2,FiraSans-Medium-8f9a781e4970d388.woff2,SourceCodePro-Regular-562dcc5011b6de7d.ttf.woff2,SourceCodePro-Semibold-d899c5a5c4aeb14a.ttf.woff2".split(",").map(f=>`<link rel="preload" as="font" type="font/woff2" crossorigin href="../../../static.files/${f}">`).join(""))</script><link rel="stylesheet" href="../../../static.files/normalize-76eba96aa4d2e634.css"><link rel="stylesheet" href="../../../static.files/rustdoc-081576b923113409.css"><meta name="rustdoc-vars" data-root-path="../../../" data-static-root-path="../../../static.files/" data-current-crate="ark_bn254" data-themes="" data-resource-suffix="" data-rustdoc-version="1.79.0 (129f3b996 2024-06-10)" data-channel="1.79.0" data-search-js="search-bf21c90c8c1d92b1.js" data-settings-js="settings-4313503d2e1961c2.js" ><script src="../../../static.files/storage-e32f0c247825364d.js"></script><script defer src="../../../static.files/src-script-e66d777a5a92e9b2.js"></script><script defer src="../../../src-files.js"></script><script defer src="../../../static.files/main-20a3ad099b048cf2.js"></script><noscript><link rel="stylesheet" href="../../../static.files/noscript-09095024cf37855e.css"></noscript><link rel="alternate icon" type="image/png" href="../../../static.files/favicon-32x32-422f7d1d52889060.png"><link rel="icon" type="image/svg+xml" href="../../../static.files/favicon-2c020d218678b618.svg"></head><body class="rustdoc src"><!--[if lte IE 11]><div class="warning">This old browser is unsupported and will most likely display funky things.</div><![endif]--><nav class="sidebar"><div class="src-sidebar-title"><h2>Files</h2></div></nav><div class="sidebar-resizer"></div><main><nav class="sub"><form class="search-form"><span></span><div id="sidebar-button" tabindex="-1"><a href="../../../ark_bn254/all.html" title="show sidebar"></a></div><input class="search-input" name="search" aria-label="Run search in the documentation" autocomplete="off" spellcheck="false" placeholder="Type ‘S’ or ‘/’ to search, ‘?’ for more options…" type="search"><div id="help-button" tabindex="-1"><a href="../../../help.html" title="help">?</a></div><div id="settings-menu" tabindex="-1"><a href="../../../settings.html" title="settings">Settings</a></div></form></nav><section id="main-content" class="content"><div class="example-wrap"><div data-nosnippet><pre class="src-line-numbers"><a href="#1" id="1">1</a>
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
</pre></div><pre class="rust"><code><span class="kw">use </span>ark_ec::{
    models::{short_weierstrass::SWCurveConfig, CurveConfig},
    short_weierstrass::Affine,
};
<span class="kw">use </span>ark_ff::{Field, MontFp, Zero};

<span class="kw">use crate</span>::{Fq, Fq2, Fr};

<span class="kw">pub type </span>G2Affine = Affine&lt;Config&gt;;

<span class="attr">#[derive(Clone, Default, PartialEq, Eq)]
</span><span class="kw">pub struct </span>Config;

<span class="kw">impl </span>CurveConfig <span class="kw">for </span>Config {
    <span class="kw">type </span>BaseField = Fq2;
    <span class="kw">type </span>ScalarField = Fr;

    <span class="doccomment">/// COFACTOR = (36 * X^4) + (36 * X^3) + (30 * X^2) + 6*X + 1
    /// 21888242871839275222246405745257275088844257914179612981679871602714643921549
    </span><span class="attr">#[rustfmt::skip]
    </span><span class="kw">const </span>COFACTOR: <span class="kw-2">&amp;</span><span class="lifetime">'static </span>[u64] = <span class="kw-2">&amp;</span>[
        <span class="number">0x345f2299c0f9fa8d</span>,
        <span class="number">0x06ceecda572a2489</span>,
        <span class="number">0xb85045b68181585e</span>,
        <span class="number">0x30644e72e131a029</span>,
    ];

    <span class="doccomment">/// COFACTOR_INV = COFACTOR^{-1} mod r
    </span><span class="kw">const </span>COFACTOR_INV: Fr =
        <span class="macro">MontFp!</span>(<span class="string">"10944121435919637613327163357776759465618812564592884533313067514031822496649"</span>);
}

<span class="kw">impl </span>SWCurveConfig <span class="kw">for </span>Config {
    <span class="doccomment">/// COEFF_A = [0, 0]
    </span><span class="kw">const </span>COEFF_A: Fq2 = Fq2::ZERO;

    <span class="doccomment">/// COEFF_B = 3/(u+9)
    /// (19485874751759354771024239261021720505790618469301721065564631296452457478373, 266929791119991161246907387137283842545076965332900288569378510910307636690)
    </span><span class="kw">const </span>COEFF_B: Fq2 = Fq2::new(
        <span class="macro">MontFp!</span>(<span class="string">"19485874751759354771024239261021720505790618469301721065564631296452457478373"</span>),
        <span class="macro">MontFp!</span>(<span class="string">"266929791119991161246907387137283842545076965332900288569378510910307636690"</span>),
    );

    <span class="doccomment">/// AFFINE_GENERATOR_COEFFS = (G2_GENERATOR_X, G2_GENERATOR_Y)
    </span><span class="kw">const </span>GENERATOR: G2Affine = G2Affine::new_unchecked(G2_GENERATOR_X, G2_GENERATOR_Y);

    <span class="attr">#[inline(always)]
    </span><span class="kw">fn </span>mul_by_a(<span class="kw">_</span>: <span class="self">Self</span>::BaseField) -&gt; <span class="self">Self</span>::BaseField {
        <span class="self">Self</span>::BaseField::zero()
    }
}

<span class="kw">pub const </span>G2_GENERATOR_X: Fq2 = Fq2::new(G2_GENERATOR_X_C0, G2_GENERATOR_X_C1);
<span class="kw">pub const </span>G2_GENERATOR_Y: Fq2 = Fq2::new(G2_GENERATOR_Y_C0, G2_GENERATOR_Y_C1);

<span class="doccomment">/// G2_GENERATOR_X_C0 =
/// 10857046999023057135944570762232829481370756359578518086990519993285655852781
</span><span class="kw">pub const </span>G2_GENERATOR_X_C0: Fq =
    <span class="macro">MontFp!</span>(<span class="string">"10857046999023057135944570762232829481370756359578518086990519993285655852781"</span>);

<span class="doccomment">/// G2_GENERATOR_X_C1 =
/// 11559732032986387107991004021392285783925812861821192530917403151452391805634
</span><span class="kw">pub const </span>G2_GENERATOR_X_C1: Fq =
    <span class="macro">MontFp!</span>(<span class="string">"11559732032986387107991004021392285783925812861821192530917403151452391805634"</span>);

<span class="doccomment">/// G2_GENERATOR_Y_C0 =
/// 8495653923123431417604973247489272438418190587263600148770280649306958101930
</span><span class="kw">pub const </span>G2_GENERATOR_Y_C0: Fq =
    <span class="macro">MontFp!</span>(<span class="string">"8495653923123431417604973247489272438418190587263600148770280649306958101930"</span>);

<span class="doccomment">/// G2_GENERATOR_Y_C1 =
/// 4082367875863433681332203403145435568316851327593401208105741076214120093531
</span><span class="kw">pub const </span>G2_GENERATOR_Y_C1: Fq =
    <span class="macro">MontFp!</span>(<span class="string">"4082367875863433681332203403145435568316851327593401208105741076214120093531"</span>);
</code></pre></div></section></main></body></html>