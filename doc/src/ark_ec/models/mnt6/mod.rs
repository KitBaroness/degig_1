<!DOCTYPE html><html lang="en"><head><meta charset="utf-8"><meta name="viewport" content="width=device-width, initial-scale=1.0"><meta name="generator" content="rustdoc"><meta name="description" content="Source of the Rust file `/Users/jeyakumar/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ark-ec-0.4.2/src/models/mnt6/mod.rs`."><title>mod.rs - source</title><script>if(window.location.protocol!=="file:")document.head.insertAdjacentHTML("beforeend","SourceSerif4-Regular-46f98efaafac5295.ttf.woff2,FiraSans-Regular-018c141bf0843ffd.woff2,FiraSans-Medium-8f9a781e4970d388.woff2,SourceCodePro-Regular-562dcc5011b6de7d.ttf.woff2,SourceCodePro-Semibold-d899c5a5c4aeb14a.ttf.woff2".split(",").map(f=>`<link rel="preload" as="font" type="font/woff2" crossorigin href="../../../../static.files/${f}">`).join(""))</script><link rel="stylesheet" href="../../../../static.files/normalize-76eba96aa4d2e634.css"><link rel="stylesheet" href="../../../../static.files/rustdoc-081576b923113409.css"><meta name="rustdoc-vars" data-root-path="../../../../" data-static-root-path="../../../../static.files/" data-current-crate="ark_ec" data-themes="" data-resource-suffix="" data-rustdoc-version="1.79.0 (129f3b996 2024-06-10)" data-channel="1.79.0" data-search-js="search-bf21c90c8c1d92b1.js" data-settings-js="settings-4313503d2e1961c2.js" ><script src="../../../../static.files/storage-e32f0c247825364d.js"></script><script defer src="../../../../static.files/src-script-e66d777a5a92e9b2.js"></script><script defer src="../../../../src-files.js"></script><script defer src="../../../../static.files/main-20a3ad099b048cf2.js"></script><noscript><link rel="stylesheet" href="../../../../static.files/noscript-09095024cf37855e.css"></noscript><link rel="alternate icon" type="image/png" href="../../../../static.files/favicon-32x32-422f7d1d52889060.png"><link rel="icon" type="image/svg+xml" href="../../../../static.files/favicon-2c020d218678b618.svg"></head><body class="rustdoc src"><!--[if lte IE 11]><div class="warning">This old browser is unsupported and will most likely display funky things.</div><![endif]--><nav class="sidebar"><div class="src-sidebar-title"><h2>Files</h2></div></nav><div class="sidebar-resizer"></div><main><nav class="sub"><form class="search-form"><span></span><div id="sidebar-button" tabindex="-1"><a href="../../../../ark_ec/all.html" title="show sidebar"></a></div><input class="search-input" name="search" aria-label="Run search in the documentation" autocomplete="off" spellcheck="false" placeholder="Type ‘S’ or ‘/’ to search, ‘?’ for more options…" type="search"><div id="help-button" tabindex="-1"><a href="../../../../help.html" title="help">?</a></div><div id="settings-menu" tabindex="-1"><a href="../../../../settings.html" title="settings">Settings</a></div></form></nav><section id="main-content" class="content"><div class="example-wrap"><div data-nosnippet><pre class="src-line-numbers"><a href="#1" id="1">1</a>
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
<a href="#106" id="106">106</a>
<a href="#107" id="107">107</a>
<a href="#108" id="108">108</a>
<a href="#109" id="109">109</a>
<a href="#110" id="110">110</a>
<a href="#111" id="111">111</a>
<a href="#112" id="112">112</a>
<a href="#113" id="113">113</a>
<a href="#114" id="114">114</a>
<a href="#115" id="115">115</a>
<a href="#116" id="116">116</a>
<a href="#117" id="117">117</a>
<a href="#118" id="118">118</a>
<a href="#119" id="119">119</a>
<a href="#120" id="120">120</a>
<a href="#121" id="121">121</a>
<a href="#122" id="122">122</a>
<a href="#123" id="123">123</a>
<a href="#124" id="124">124</a>
<a href="#125" id="125">125</a>
<a href="#126" id="126">126</a>
<a href="#127" id="127">127</a>
<a href="#128" id="128">128</a>
<a href="#129" id="129">129</a>
<a href="#130" id="130">130</a>
<a href="#131" id="131">131</a>
<a href="#132" id="132">132</a>
<a href="#133" id="133">133</a>
<a href="#134" id="134">134</a>
<a href="#135" id="135">135</a>
<a href="#136" id="136">136</a>
<a href="#137" id="137">137</a>
<a href="#138" id="138">138</a>
<a href="#139" id="139">139</a>
<a href="#140" id="140">140</a>
<a href="#141" id="141">141</a>
<a href="#142" id="142">142</a>
<a href="#143" id="143">143</a>
<a href="#144" id="144">144</a>
<a href="#145" id="145">145</a>
<a href="#146" id="146">146</a>
<a href="#147" id="147">147</a>
<a href="#148" id="148">148</a>
<a href="#149" id="149">149</a>
<a href="#150" id="150">150</a>
<a href="#151" id="151">151</a>
<a href="#152" id="152">152</a>
<a href="#153" id="153">153</a>
<a href="#154" id="154">154</a>
<a href="#155" id="155">155</a>
<a href="#156" id="156">156</a>
<a href="#157" id="157">157</a>
<a href="#158" id="158">158</a>
<a href="#159" id="159">159</a>
<a href="#160" id="160">160</a>
<a href="#161" id="161">161</a>
<a href="#162" id="162">162</a>
<a href="#163" id="163">163</a>
<a href="#164" id="164">164</a>
<a href="#165" id="165">165</a>
<a href="#166" id="166">166</a>
<a href="#167" id="167">167</a>
<a href="#168" id="168">168</a>
<a href="#169" id="169">169</a>
<a href="#170" id="170">170</a>
<a href="#171" id="171">171</a>
<a href="#172" id="172">172</a>
<a href="#173" id="173">173</a>
<a href="#174" id="174">174</a>
<a href="#175" id="175">175</a>
<a href="#176" id="176">176</a>
<a href="#177" id="177">177</a>
<a href="#178" id="178">178</a>
<a href="#179" id="179">179</a>
<a href="#180" id="180">180</a>
<a href="#181" id="181">181</a>
<a href="#182" id="182">182</a>
<a href="#183" id="183">183</a>
<a href="#184" id="184">184</a>
<a href="#185" id="185">185</a>
<a href="#186" id="186">186</a>
<a href="#187" id="187">187</a>
<a href="#188" id="188">188</a>
<a href="#189" id="189">189</a>
<a href="#190" id="190">190</a>
<a href="#191" id="191">191</a>
<a href="#192" id="192">192</a>
<a href="#193" id="193">193</a>
<a href="#194" id="194">194</a>
<a href="#195" id="195">195</a>
<a href="#196" id="196">196</a>
<a href="#197" id="197">197</a>
<a href="#198" id="198">198</a>
<a href="#199" id="199">199</a>
<a href="#200" id="200">200</a>
<a href="#201" id="201">201</a>
<a href="#202" id="202">202</a>
<a href="#203" id="203">203</a>
<a href="#204" id="204">204</a>
<a href="#205" id="205">205</a>
<a href="#206" id="206">206</a>
<a href="#207" id="207">207</a>
<a href="#208" id="208">208</a>
<a href="#209" id="209">209</a>
<a href="#210" id="210">210</a>
<a href="#211" id="211">211</a>
<a href="#212" id="212">212</a>
<a href="#213" id="213">213</a>
<a href="#214" id="214">214</a>
<a href="#215" id="215">215</a>
<a href="#216" id="216">216</a>
<a href="#217" id="217">217</a>
<a href="#218" id="218">218</a>
<a href="#219" id="219">219</a>
<a href="#220" id="220">220</a>
<a href="#221" id="221">221</a>
<a href="#222" id="222">222</a>
<a href="#223" id="223">223</a>
<a href="#224" id="224">224</a>
<a href="#225" id="225">225</a>
<a href="#226" id="226">226</a>
<a href="#227" id="227">227</a>
<a href="#228" id="228">228</a>
<a href="#229" id="229">229</a>
<a href="#230" id="230">230</a>
<a href="#231" id="231">231</a>
<a href="#232" id="232">232</a>
<a href="#233" id="233">233</a>
<a href="#234" id="234">234</a>
<a href="#235" id="235">235</a>
<a href="#236" id="236">236</a>
<a href="#237" id="237">237</a>
<a href="#238" id="238">238</a>
<a href="#239" id="239">239</a>
<a href="#240" id="240">240</a>
<a href="#241" id="241">241</a>
<a href="#242" id="242">242</a>
<a href="#243" id="243">243</a>
<a href="#244" id="244">244</a>
<a href="#245" id="245">245</a>
<a href="#246" id="246">246</a>
<a href="#247" id="247">247</a>
<a href="#248" id="248">248</a>
<a href="#249" id="249">249</a>
<a href="#250" id="250">250</a>
<a href="#251" id="251">251</a>
<a href="#252" id="252">252</a>
<a href="#253" id="253">253</a>
<a href="#254" id="254">254</a>
<a href="#255" id="255">255</a>
</pre></div><pre class="rust"><code><span class="kw">use crate</span>::{
    models::{short_weierstrass::SWCurveConfig, CurveConfig},
    pairing::{MillerLoopOutput, Pairing, PairingOutput},
};
<span class="kw">use </span>ark_ff::{
    fp3::{Fp3, Fp3Config},
    fp6_2over3::{Fp6, Fp6Config},
    CyclotomicMultSubgroup, Field, PrimeField,
};
<span class="kw">use </span>itertools::Itertools;
<span class="kw">use </span>num_traits::{One, Zero};

<span class="kw">use </span>ark_std::{marker::PhantomData, vec::Vec};

<span class="attr">#[cfg(feature = <span class="string">"parallel"</span>)]
</span><span class="kw">use </span>rayon::prelude::<span class="kw-2">*</span>;

<span class="kw">pub mod </span>g1;
<span class="kw">pub mod </span>g2;

<span class="kw">use </span><span class="self">self</span>::g2::{AteAdditionCoefficients, AteDoubleCoefficients, G2ProjectiveExtended};
<span class="kw">pub use </span><span class="self">self</span>::{
    g1::{G1Affine, G1Prepared, G1Projective},
    g2::{G2Affine, G2Prepared, G2Projective},
};

<span class="kw">pub type </span>GT&lt;P&gt; = Fp6&lt;P&gt;;

<span class="kw">pub trait </span>MNT6Config: <span class="lifetime">'static </span>+ Sized {
    <span class="kw">const </span>TWIST: Fp3&lt;<span class="self">Self</span>::Fp3Config&gt;;
    <span class="kw">const </span>TWIST_COEFF_A: Fp3&lt;<span class="self">Self</span>::Fp3Config&gt;;
    <span class="kw">const </span>ATE_LOOP_COUNT: <span class="kw-2">&amp;</span><span class="lifetime">'static </span>[i8];
    <span class="kw">const </span>ATE_IS_LOOP_COUNT_NEG: bool;
    <span class="kw">const </span>FINAL_EXPONENT_LAST_CHUNK_1: &lt;<span class="self">Self</span>::Fp <span class="kw">as </span>PrimeField&gt;::BigInt;
    <span class="kw">const </span>FINAL_EXPONENT_LAST_CHUNK_W0_IS_NEG: bool;
    <span class="kw">const </span>FINAL_EXPONENT_LAST_CHUNK_ABS_OF_W0: &lt;<span class="self">Self</span>::Fp <span class="kw">as </span>PrimeField&gt;::BigInt;
    <span class="kw">type </span>Fp: PrimeField + Into&lt;&lt;<span class="self">Self</span>::Fp <span class="kw">as </span>PrimeField&gt;::BigInt&gt;;
    <span class="kw">type </span>Fr: PrimeField + Into&lt;&lt;<span class="self">Self</span>::Fr <span class="kw">as </span>PrimeField&gt;::BigInt&gt;;
    <span class="kw">type </span>Fp3Config: Fp3Config&lt;Fp = <span class="self">Self</span>::Fp&gt;;
    <span class="kw">type </span>Fp6Config: Fp6Config&lt;Fp3Config = <span class="self">Self</span>::Fp3Config&gt;;
    <span class="kw">type </span>G1Config: SWCurveConfig&lt;BaseField = <span class="self">Self</span>::Fp, ScalarField = <span class="self">Self</span>::Fr&gt;;
    <span class="kw">type </span>G2Config: SWCurveConfig&lt;
        BaseField = Fp3&lt;<span class="self">Self</span>::Fp3Config&gt;,
        ScalarField = &lt;<span class="self">Self</span>::G1Config <span class="kw">as </span>CurveConfig&gt;::ScalarField,
    &gt;;

    <span class="kw">fn </span>multi_miller_loop(
        a: <span class="kw">impl </span>IntoIterator&lt;Item = <span class="kw">impl </span>Into&lt;G1Prepared&lt;<span class="self">Self</span>&gt;&gt;&gt;,
        b: <span class="kw">impl </span>IntoIterator&lt;Item = <span class="kw">impl </span>Into&lt;G2Prepared&lt;<span class="self">Self</span>&gt;&gt;&gt;,
    ) -&gt; MillerLoopOutput&lt;MNT6&lt;<span class="self">Self</span>&gt;&gt; {
        <span class="kw">let </span>pairs = a
            .into_iter()
            .zip_eq(b)
            .map(|(a, b)| (a.into(), b.into()))
            .collect::&lt;Vec&lt;<span class="kw">_</span>&gt;&gt;();
        <span class="kw">let </span>result = <span class="macro">cfg_into_iter!</span>(pairs)
            .map(|(a, b)| MNT6::&lt;<span class="self">Self</span>&gt;::ate_miller_loop(<span class="kw-2">&amp;</span>a, <span class="kw-2">&amp;</span>b))
            .product();
        MillerLoopOutput(result)
    }

    <span class="kw">fn </span>final_exponentiation(f: MillerLoopOutput&lt;MNT6&lt;<span class="self">Self</span>&gt;&gt;) -&gt; <span class="prelude-ty">Option</span>&lt;PairingOutput&lt;MNT6&lt;<span class="self">Self</span>&gt;&gt;&gt; {
        <span class="kw">let </span>value = f.<span class="number">0</span>;
        <span class="kw">let </span>value_inv = value.inverse()<span class="question-mark">?</span>;
        <span class="kw">let </span>value_to_first_chunk =
            MNT6::&lt;<span class="self">Self</span>&gt;::final_exponentiation_first_chunk(<span class="kw-2">&amp;</span>value, <span class="kw-2">&amp;</span>value_inv);
        <span class="kw">let </span>value_inv_to_first_chunk =
            MNT6::&lt;<span class="self">Self</span>&gt;::final_exponentiation_first_chunk(<span class="kw-2">&amp;</span>value_inv, <span class="kw-2">&amp;</span>value);
        <span class="kw">let </span>result = MNT6::&lt;<span class="self">Self</span>&gt;::final_exponentiation_last_chunk(
            <span class="kw-2">&amp;</span>value_to_first_chunk,
            <span class="kw-2">&amp;</span>value_inv_to_first_chunk,
        );
        <span class="prelude-val">Some</span>(PairingOutput(result))
    }
}

<span class="attr">#[derive(Derivative)]
#[derivative(Copy, Clone, PartialEq, Eq, Debug, Hash)]
</span><span class="kw">pub struct </span>MNT6&lt;P: MNT6Config&gt;(PhantomData&lt;<span class="kw">fn</span>() -&gt; P&gt;);

<span class="kw">impl</span>&lt;P: MNT6Config&gt; MNT6&lt;P&gt; {
    <span class="kw">fn </span>doubling_for_flipped_miller_loop(
        r: <span class="kw-2">&amp;</span>G2ProjectiveExtended&lt;P&gt;,
    ) -&gt; (G2ProjectiveExtended&lt;P&gt;, AteDoubleCoefficients&lt;P&gt;) {
        <span class="kw">let </span>a = r.t.square();
        <span class="kw">let </span>b = r.x.square();
        <span class="kw">let </span>c = r.y.square();
        <span class="kw">let </span>d = c.square();
        <span class="kw">let </span>e = (r.x + <span class="kw-2">&amp;</span>c).square() - <span class="kw-2">&amp;</span>b - <span class="kw-2">&amp;</span>d;
        <span class="kw">let </span>f = (b + <span class="kw-2">&amp;</span>b + <span class="kw-2">&amp;</span>b) + <span class="kw-2">&amp;</span>(P::TWIST_COEFF_A * <span class="kw-2">&amp;</span>a);
        <span class="kw">let </span>g = f.square();

        <span class="kw">let </span>d_eight = d.double().double().double();

        <span class="kw">let </span>e2 = e.double();
        <span class="kw">let </span>x = g - <span class="kw-2">&amp;</span>e2.double();
        <span class="kw">let </span>y = -d_eight + <span class="kw-2">&amp;</span>(f * <span class="kw-2">&amp;</span>(e2 - <span class="kw-2">&amp;</span>x));
        <span class="kw">let </span>z = (r.y + <span class="kw-2">&amp;</span>r.z).square() - <span class="kw-2">&amp;</span>c - <span class="kw-2">&amp;</span>r.z.square();
        <span class="kw">let </span>t = z.square();

        <span class="kw">let </span>r2 = G2ProjectiveExtended { x, y, z, t };
        <span class="kw">let </span>coeff = AteDoubleCoefficients {
            c_h: (r2.z + <span class="kw-2">&amp;</span>r.t).square() - <span class="kw-2">&amp;</span>r2.t - <span class="kw-2">&amp;</span>a,
            c_4c: c + <span class="kw-2">&amp;</span>c + <span class="kw-2">&amp;</span>c + <span class="kw-2">&amp;</span>c,
            c_j: (f + <span class="kw-2">&amp;</span>r.t).square() - <span class="kw-2">&amp;</span>g - <span class="kw-2">&amp;</span>a,
            c_l: (f + <span class="kw-2">&amp;</span>r.x).square() - <span class="kw-2">&amp;</span>g - <span class="kw-2">&amp;</span>b,
        };

        (r2, coeff)
    }

    <span class="kw">fn </span>mixed_addition_for_flipper_miller_loop(
        x: <span class="kw-2">&amp;</span>Fp3&lt;P::Fp3Config&gt;,
        y: <span class="kw-2">&amp;</span>Fp3&lt;P::Fp3Config&gt;,
        r: <span class="kw-2">&amp;</span>G2ProjectiveExtended&lt;P&gt;,
    ) -&gt; (G2ProjectiveExtended&lt;P&gt;, AteAdditionCoefficients&lt;P&gt;) {
        <span class="kw">let </span>a = y.square();
        <span class="kw">let </span>b = r.t * x;
        <span class="kw">let </span>d = ((r.z + y).square() - <span class="kw-2">&amp;</span>a - <span class="kw-2">&amp;</span>r.t) * <span class="kw-2">&amp;</span>r.t;
        <span class="kw">let </span>h = b - <span class="kw-2">&amp;</span>r.x;
        <span class="kw">let </span>i = h.square();
        <span class="kw">let </span>e = i + <span class="kw-2">&amp;</span>i + <span class="kw-2">&amp;</span>i + <span class="kw-2">&amp;</span>i;
        <span class="kw">let </span>j = h * <span class="kw-2">&amp;</span>e;
        <span class="kw">let </span>v = r.x * <span class="kw-2">&amp;</span>e;
        <span class="kw">let </span>ry2 = r.y.double();
        <span class="kw">let </span>l1 = d - <span class="kw-2">&amp;</span>ry2;

        <span class="kw">let </span>x = l1.square() - <span class="kw-2">&amp;</span>j - <span class="kw-2">&amp;</span>(v + <span class="kw-2">&amp;</span>v);
        <span class="kw">let </span>y = l1 * <span class="kw-2">&amp;</span>(v - <span class="kw-2">&amp;</span>x) - <span class="kw-2">&amp;</span>(j * <span class="kw-2">&amp;</span>ry2);
        <span class="kw">let </span>z = (r.z + <span class="kw-2">&amp;</span>h).square() - <span class="kw-2">&amp;</span>r.t - <span class="kw-2">&amp;</span>i;
        <span class="kw">let </span>t = z.square();

        <span class="kw">let </span>r2 = G2ProjectiveExtended { x, y, z, t };
        <span class="kw">let </span>coeff = AteAdditionCoefficients { c_l1: l1, c_rz: z };

        (r2, coeff)
    }

    <span class="kw">pub fn </span>ate_miller_loop(p: <span class="kw-2">&amp;</span>G1Prepared&lt;P&gt;, q: <span class="kw-2">&amp;</span>G2Prepared&lt;P&gt;) -&gt; Fp6&lt;P::Fp6Config&gt; {
        <span class="kw">let </span>l1_coeff = Fp3::new(p.x, P::Fp::zero(), P::Fp::zero()) - <span class="kw-2">&amp;</span>q.x_over_twist;

        <span class="kw">let </span><span class="kw-2">mut </span>f = &lt;Fp6&lt;P::Fp6Config&gt;&gt;::one();

        <span class="kw">let </span><span class="kw-2">mut </span>add_idx: usize = <span class="number">0</span>;

        <span class="comment">// code below gets executed for all bits (EXCEPT the MSB itself) of
        // mnt6_param_p (skipping leading zeros) in MSB to LSB order
        </span><span class="kw">let </span>y_over_twist_neg = -q.y_over_twist;
        <span class="macro">assert_eq!</span>(P::ATE_LOOP_COUNT.len() - <span class="number">1</span>, q.double_coefficients.len());
        <span class="kw">for </span>(bit, dc) <span class="kw">in </span>P::ATE_LOOP_COUNT.iter().skip(<span class="number">1</span>).zip(<span class="kw-2">&amp;</span>q.double_coefficients) {
            <span class="kw">let </span>g_rr_at_p = Fp6::new(
                dc.c_l - <span class="kw-2">&amp;</span>dc.c_4c - <span class="kw-2">&amp;</span>(dc.c_j * <span class="kw-2">&amp;</span>p.x_twist),
                dc.c_h * <span class="kw-2">&amp;</span>p.y_twist,
            );

            f = f.square() * <span class="kw-2">&amp;</span>g_rr_at_p;

            <span class="comment">// Compute l_{R,Q}(P) if bit == 1, and l_{R,-Q}(P) if bit == -1
            </span><span class="kw">let </span>g_rq_at_p = <span class="kw">if </span><span class="kw-2">*</span>bit == <span class="number">1 </span>{
                <span class="kw">let </span>ac = <span class="kw-2">&amp;</span>q.addition_coefficients[add_idx];
                add_idx += <span class="number">1</span>;

                Fp6::new(
                    ac.c_rz * <span class="kw-2">&amp;</span>p.y_twist,
                    -(q.y_over_twist * <span class="kw-2">&amp;</span>ac.c_rz + <span class="kw-2">&amp;</span>(l1_coeff * <span class="kw-2">&amp;</span>ac.c_l1)),
                )
            } <span class="kw">else if </span><span class="kw-2">*</span>bit == -<span class="number">1 </span>{
                <span class="kw">let </span>ac = <span class="kw-2">&amp;</span>q.addition_coefficients[add_idx];
                add_idx += <span class="number">1</span>;
                Fp6::new(
                    ac.c_rz * <span class="kw-2">&amp;</span>p.y_twist,
                    -(y_over_twist_neg * <span class="kw-2">&amp;</span>ac.c_rz + <span class="kw-2">&amp;</span>(l1_coeff * <span class="kw-2">&amp;</span>ac.c_l1)),
                )
            } <span class="kw">else if </span><span class="kw-2">*</span>bit == <span class="number">0 </span>{
                <span class="kw">continue</span>;
            } <span class="kw">else </span>{
                <span class="macro">unreachable!</span>();
            };
            f <span class="kw-2">*</span>= <span class="kw-2">&amp;</span>g_rq_at_p;
        }

        <span class="kw">if </span>P::ATE_IS_LOOP_COUNT_NEG {
            <span class="kw">let </span>ac = <span class="kw-2">&amp;</span>q.addition_coefficients[add_idx];

            <span class="kw">let </span>g_rnegr_at_p = Fp6::new(
                ac.c_rz * <span class="kw-2">&amp;</span>p.y_twist,
                -(q.y_over_twist * <span class="kw-2">&amp;</span>ac.c_rz + <span class="kw-2">&amp;</span>(l1_coeff * <span class="kw-2">&amp;</span>ac.c_l1)),
            );
            f = (f * <span class="kw-2">&amp;</span>g_rnegr_at_p).inverse().unwrap();
        }

        f
    }

    <span class="kw">fn </span>final_exponentiation_first_chunk(
        elt: <span class="kw-2">&amp;</span>Fp6&lt;P::Fp6Config&gt;,
        elt_inv: <span class="kw-2">&amp;</span>Fp6&lt;P::Fp6Config&gt;,
    ) -&gt; Fp6&lt;P::Fp6Config&gt; {
        <span class="comment">// (q^3-1)*(q+1)

        // elt_q3 = elt^(q^3)
        </span><span class="kw">let </span><span class="kw-2">mut </span>elt_q3 = <span class="kw-2">*</span>elt;
        elt_q3.cyclotomic_inverse_in_place();
        <span class="comment">// elt_q3_over_elt = elt^(q^3-1)
        </span><span class="kw">let </span>elt_q3_over_elt = elt_q3 * elt_inv;
        <span class="comment">// alpha = elt^((q^3-1) * q)
        </span><span class="kw">let </span><span class="kw-2">mut </span>alpha = elt_q3_over_elt;
        alpha.frobenius_map_in_place(<span class="number">1</span>);
        <span class="comment">// beta = elt^((q^3-1)*(q+1)
        </span>alpha * <span class="kw-2">&amp;</span>elt_q3_over_elt
    }

    <span class="kw">fn </span>final_exponentiation_last_chunk(
        elt: <span class="kw-2">&amp;</span>Fp6&lt;P::Fp6Config&gt;,
        elt_inv: <span class="kw-2">&amp;</span>Fp6&lt;P::Fp6Config&gt;,
    ) -&gt; Fp6&lt;P::Fp6Config&gt; {
        <span class="kw">let </span>elt_clone = <span class="kw-2">*</span>elt;
        <span class="kw">let </span>elt_inv_clone = <span class="kw-2">*</span>elt_inv;

        <span class="kw">let </span><span class="kw-2">mut </span>elt_q = <span class="kw-2">*</span>elt;
        elt_q.frobenius_map_in_place(<span class="number">1</span>);

        <span class="kw">let </span>w1_part = elt_q.cyclotomic_exp(P::FINAL_EXPONENT_LAST_CHUNK_1);
        <span class="kw">let </span>w0_part = <span class="kw">if </span>P::FINAL_EXPONENT_LAST_CHUNK_W0_IS_NEG {
            elt_inv_clone.cyclotomic_exp(P::FINAL_EXPONENT_LAST_CHUNK_ABS_OF_W0)
        } <span class="kw">else </span>{
            elt_clone.cyclotomic_exp(P::FINAL_EXPONENT_LAST_CHUNK_ABS_OF_W0)
        };

        w1_part * <span class="kw-2">&amp;</span>w0_part
    }
}

<span class="kw">impl</span>&lt;P: MNT6Config&gt; Pairing <span class="kw">for </span>MNT6&lt;P&gt; {
    <span class="kw">type </span>BaseField = &lt;P::G1Config <span class="kw">as </span>CurveConfig&gt;::BaseField;
    <span class="kw">type </span>ScalarField = &lt;P::G1Config <span class="kw">as </span>CurveConfig&gt;::ScalarField;
    <span class="kw">type </span>G1 = G1Projective&lt;P&gt;;
    <span class="kw">type </span>G1Affine = G1Affine&lt;P&gt;;
    <span class="kw">type </span>G1Prepared = G1Prepared&lt;P&gt;;
    <span class="kw">type </span>G2 = G2Projective&lt;P&gt;;
    <span class="kw">type </span>G2Affine = G2Affine&lt;P&gt;;
    <span class="kw">type </span>G2Prepared = G2Prepared&lt;P&gt;;
    <span class="kw">type </span>TargetField = Fp6&lt;P::Fp6Config&gt;;

    <span class="kw">fn </span>multi_miller_loop(
        a: <span class="kw">impl </span>IntoIterator&lt;Item = <span class="kw">impl </span>Into&lt;<span class="self">Self</span>::G1Prepared&gt;&gt;,
        b: <span class="kw">impl </span>IntoIterator&lt;Item = <span class="kw">impl </span>Into&lt;<span class="self">Self</span>::G2Prepared&gt;&gt;,
    ) -&gt; MillerLoopOutput&lt;<span class="self">Self</span>&gt; {
        P::multi_miller_loop(a, b)
    }

    <span class="kw">fn </span>final_exponentiation(f: MillerLoopOutput&lt;<span class="self">Self</span>&gt;) -&gt; <span class="prelude-ty">Option</span>&lt;PairingOutput&lt;<span class="self">Self</span>&gt;&gt; {
        P::final_exponentiation(f)
    }
}
</code></pre></div></section></main></body></html>