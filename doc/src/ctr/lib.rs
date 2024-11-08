<!DOCTYPE html><html lang="en"><head><meta charset="utf-8"><meta name="viewport" content="width=device-width, initial-scale=1.0"><meta name="generator" content="rustdoc"><meta name="description" content="Source of the Rust file `/Users/jeyakumar/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ctr-0.8.0/src/lib.rs`."><title>lib.rs - source</title><script>if(window.location.protocol!=="file:")document.head.insertAdjacentHTML("beforeend","SourceSerif4-Regular-46f98efaafac5295.ttf.woff2,FiraSans-Regular-018c141bf0843ffd.woff2,FiraSans-Medium-8f9a781e4970d388.woff2,SourceCodePro-Regular-562dcc5011b6de7d.ttf.woff2,SourceCodePro-Semibold-d899c5a5c4aeb14a.ttf.woff2".split(",").map(f=>`<link rel="preload" as="font" type="font/woff2" crossorigin href="../../static.files/${f}">`).join(""))</script><link rel="stylesheet" href="../../static.files/normalize-76eba96aa4d2e634.css"><link rel="stylesheet" href="../../static.files/rustdoc-081576b923113409.css"><meta name="rustdoc-vars" data-root-path="../../" data-static-root-path="../../static.files/" data-current-crate="ctr" data-themes="" data-resource-suffix="" data-rustdoc-version="1.79.0 (129f3b996 2024-06-10)" data-channel="1.79.0" data-search-js="search-bf21c90c8c1d92b1.js" data-settings-js="settings-4313503d2e1961c2.js" ><script src="../../static.files/storage-e32f0c247825364d.js"></script><script defer src="../../static.files/src-script-e66d777a5a92e9b2.js"></script><script defer src="../../src-files.js"></script><script defer src="../../static.files/main-20a3ad099b048cf2.js"></script><noscript><link rel="stylesheet" href="../../static.files/noscript-09095024cf37855e.css"></noscript><link rel="icon" href="https://raw.githubusercontent.com/RustCrypto/meta/master/logo.svg"></head><body class="rustdoc src"><!--[if lte IE 11]><div class="warning">This old browser is unsupported and will most likely display funky things.</div><![endif]--><nav class="sidebar"><div class="src-sidebar-title"><h2>Files</h2></div></nav><div class="sidebar-resizer"></div><main><nav class="sub"><form class="search-form"><span></span><div id="sidebar-button" tabindex="-1"><a href="../../ctr/all.html" title="show sidebar"></a></div><input class="search-input" name="search" aria-label="Run search in the documentation" autocomplete="off" spellcheck="false" placeholder="Type ‘S’ or ‘/’ to search, ‘?’ for more options…" type="search"><div id="help-button" tabindex="-1"><a href="../../help.html" title="help">?</a></div><div id="settings-menu" tabindex="-1"><a href="../../settings.html" title="settings">Settings</a></div></form></nav><section id="main-content" class="content"><div class="example-wrap"><div data-nosnippet><pre class="src-line-numbers"><a href="#1" id="1">1</a>
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
</pre></div><pre class="rust"><code><span class="doccomment">//! Generic implementations of CTR mode for block ciphers.
//!
//! Mode functionality is accessed using traits from re-exported
//! [`cipher`](https://docs.rs/cipher) crate.
//!
//! # ⚠️ Security Warning: [Hazmat!]
//!
//! This crate does not ensure ciphertexts are authentic! Thus ciphertext integrity
//! is not verified, which can lead to serious vulnerabilities!
//!
//! # `Ctr128` Usage Example
//!
//! ```
//! use ctr::cipher::{NewCipher, StreamCipher, StreamCipherSeek};
//!
//! // `aes` crate provides AES block cipher implementation
//! type Aes128Ctr = ctr::Ctr128BE&lt;aes::Aes128&gt;;
//!
//! let mut data = [1, 2, 3, 4, 5, 6, 7];
//!
//! let key = b"very secret key.";
//! let nonce = b"and secret nonce";
//!
//! // create cipher instance
//! let mut cipher = Aes128Ctr::new(key.into(), nonce.into());
//!
//! // apply keystream (encrypt)
//! cipher.apply_keystream(&amp;mut data);
//! assert_eq!(data, [6, 245, 126, 124, 180, 146, 37]);
//!
//! // seek to the keystream beginning and apply it again to the `data` (decrypt)
//! cipher.seek(0);
//! cipher.apply_keystream(&amp;mut data);
//! assert_eq!(data, [1, 2, 3, 4, 5, 6, 7]);
//! ```
//!
//! [Hazmat!]: https://github.com/RustCrypto/meta/blob/master/HAZMAT.md

</span><span class="attr">#![no_std]
#![doc(
    html_logo_url = <span class="string">"https://raw.githubusercontent.com/RustCrypto/meta/master/logo.svg"</span>,
    html_favicon_url = <span class="string">"https://raw.githubusercontent.com/RustCrypto/meta/master/logo.svg"
</span>)]
#![warn(missing_docs, rust_2018_idioms)]

</span><span class="kw">pub use </span>cipher;
<span class="kw">use </span>cipher::{
    errors::{LoopError, OverflowError},
    generic_array::typenum::Unsigned,
    Block, BlockEncrypt, FromBlockCipher, ParBlocks, SeekNum, StreamCipher, StreamCipherSeek,
};
<span class="kw">use </span>core::fmt;

<span class="kw">pub mod </span>flavors;
<span class="kw">use </span>flavors::CtrFlavor;

<span class="doccomment">/// CTR mode with 128-bit big endian counter.
</span><span class="kw">pub type </span>Ctr128BE&lt;B&gt; = Ctr&lt;B, flavors::Ctr128BE&gt;;
<span class="doccomment">/// CTR mode with 128-bit little endian counter.
</span><span class="kw">pub type </span>Ctr128LE&lt;B&gt; = Ctr&lt;B, flavors::Ctr128LE&gt;;
<span class="doccomment">/// CTR mode with 64-bit big endian counter.
</span><span class="kw">pub type </span>Ctr64BE&lt;B&gt; = Ctr&lt;B, flavors::Ctr64BE&gt;;
<span class="doccomment">/// CTR mode with 64-bit little endian counter.
</span><span class="kw">pub type </span>Ctr64LE&lt;B&gt; = Ctr&lt;B, flavors::Ctr64LE&gt;;
<span class="doccomment">/// CTR mode with 32-bit big endian counter.
</span><span class="kw">pub type </span>Ctr32BE&lt;B&gt; = Ctr&lt;B, flavors::Ctr32BE&gt;;
<span class="doccomment">/// CTR mode with 32-bit little endian counter.
</span><span class="kw">pub type </span>Ctr32LE&lt;B&gt; = Ctr&lt;B, flavors::Ctr32LE&gt;;

<span class="doccomment">/// Generic CTR block mode isntance.
</span><span class="attr">#[derive(Clone)]
</span><span class="kw">pub struct </span>Ctr&lt;B, F&gt;
<span class="kw">where
    </span>B: BlockEncrypt,
    F: CtrFlavor&lt;B::BlockSize&gt;,
{
    cipher: B,
    nonce: &lt;F <span class="kw">as </span>CtrFlavor&lt;B::BlockSize&gt;&gt;::Nonce,
    counter: F,
    buffer: Block&lt;B&gt;,
    buf_pos: u8,
}

<span class="kw">impl</span>&lt;B, F&gt; Ctr&lt;B, F&gt;
<span class="kw">where
    </span>B: BlockEncrypt,
    F: CtrFlavor&lt;B::BlockSize&gt;,
{
    <span class="kw">fn </span>check_data_len(<span class="kw-2">&amp;</span><span class="self">self</span>, data: <span class="kw-2">&amp;</span>[u8]) -&gt; <span class="prelude-ty">Result</span>&lt;(), LoopError&gt; {
        <span class="kw">let </span>bs = B::BlockSize::USIZE;
        <span class="kw">let </span>leftover_bytes = bs - <span class="self">self</span>.buf_pos <span class="kw">as </span>usize;
        <span class="kw">if </span>data.len() &lt; leftover_bytes {
            <span class="kw">return </span><span class="prelude-val">Ok</span>(());
        }
        <span class="kw">let </span>blocks = <span class="number">1 </span>+ (data.len() - leftover_bytes) / bs;
        <span class="self">self</span>.counter
            .checked_add(blocks)
            .ok_or(LoopError)
            .map(|<span class="kw">_</span>| ())
    }

    <span class="doccomment">/// Seek to the given block
    </span><span class="comment">// TODO: replace with a trait-based method
    </span><span class="kw">pub fn </span>seek_block(<span class="kw-2">&amp;mut </span><span class="self">self</span>, block: F::Backend) {
        <span class="self">self</span>.counter = F::from_backend(block);
    }

    <span class="doccomment">/// Return number of the current block
    </span><span class="comment">// TODO: replace with a trait-based method
    </span><span class="kw">pub fn </span>current_block(<span class="kw-2">&amp;</span><span class="self">self</span>) -&gt; F::Backend {
        <span class="self">self</span>.counter.to_backend()
    }
}

<span class="kw">impl</span>&lt;B, F&gt; FromBlockCipher <span class="kw">for </span>Ctr&lt;B, F&gt;
<span class="kw">where
    </span>B: BlockEncrypt,
    F: CtrFlavor&lt;B::BlockSize&gt;,
{
    <span class="kw">type </span>BlockCipher = B;
    <span class="kw">type </span>NonceSize = B::BlockSize;

    <span class="attr">#[inline]
    </span><span class="kw">fn </span>from_block_cipher(cipher: B, nonce: <span class="kw-2">&amp;</span>Block&lt;B&gt;) -&gt; <span class="self">Self </span>{
        <span class="kw">let </span>nonce = F::load(nonce);
        <span class="self">Self </span>{
            cipher,
            buffer: Default::default(),
            nonce,
            counter: Default::default(),
            buf_pos: <span class="number">0</span>,
        }
    }
}

<span class="kw">impl</span>&lt;B, F&gt; StreamCipher <span class="kw">for </span>Ctr&lt;B, F&gt;
<span class="kw">where
    </span>B: BlockEncrypt,
    F: CtrFlavor&lt;B::BlockSize&gt;,
{
    <span class="kw">fn </span>try_apply_keystream(<span class="kw-2">&amp;mut </span><span class="self">self</span>, <span class="kw-2">mut </span>data: <span class="kw-2">&amp;mut </span>[u8]) -&gt; <span class="prelude-ty">Result</span>&lt;(), LoopError&gt; {
        <span class="self">self</span>.check_data_len(data)<span class="question-mark">?</span>;
        <span class="kw">let </span>bs = B::BlockSize::USIZE;
        <span class="kw">let </span>pos = <span class="self">self</span>.buf_pos <span class="kw">as </span>usize;
        <span class="macro">debug_assert!</span>(bs &gt; pos);

        <span class="kw">let </span><span class="kw-2">mut </span>counter = <span class="self">self</span>.counter.clone();
        <span class="kw">if </span>pos != <span class="number">0 </span>{
            <span class="kw">if </span>data.len() &lt; bs - pos {
                <span class="kw">let </span>n = pos + data.len();
                xor(data, <span class="kw-2">&amp;</span><span class="self">self</span>.buffer[pos..n]);
                <span class="self">self</span>.buf_pos = n <span class="kw">as </span>u8;
                <span class="kw">return </span><span class="prelude-val">Ok</span>(());
            } <span class="kw">else </span>{
                <span class="kw">let </span>(l, r) = data.split_at_mut(bs - pos);
                data = r;
                xor(l, <span class="kw-2">&amp;</span><span class="self">self</span>.buffer[pos..]);
                counter.increment();
            }
        }

        <span class="comment">// Process blocks in parallel if cipher supports it
        </span><span class="kw">let </span>pb = B::ParBlocks::USIZE;
        <span class="kw">if </span>pb != <span class="number">1 </span>{
            <span class="kw">let </span><span class="kw-2">mut </span>chunks = data.chunks_exact_mut(bs * pb);
            <span class="kw">let </span><span class="kw-2">mut </span>blocks: ParBlocks&lt;B&gt; = Default::default();
            <span class="kw">for </span>chunk <span class="kw">in </span><span class="kw-2">&amp;mut </span>chunks {
                <span class="kw">for </span>b <span class="kw">in </span>blocks.iter_mut() {
                    <span class="kw-2">*</span>b = counter.generate_block(<span class="kw-2">&amp;</span><span class="self">self</span>.nonce);
                    counter.increment();
                }

                <span class="self">self</span>.cipher.encrypt_par_blocks(<span class="kw-2">&amp;mut </span>blocks);
                xor(chunk, to_slice::&lt;B&gt;(<span class="kw-2">&amp;</span>blocks));
            }
            data = chunks.into_remainder();
        }

        <span class="kw">let </span><span class="kw-2">mut </span>chunks = data.chunks_exact_mut(bs);
        <span class="kw">for </span>chunk <span class="kw">in </span><span class="kw-2">&amp;mut </span>chunks {
            <span class="kw">let </span><span class="kw-2">mut </span>block = counter.generate_block(<span class="kw-2">&amp;</span><span class="self">self</span>.nonce);
            counter.increment();
            <span class="self">self</span>.cipher.encrypt_block(<span class="kw-2">&amp;mut </span>block);
            xor(chunk, <span class="kw-2">&amp;</span>block);
        }

        <span class="kw">let </span>rem = chunks.into_remainder();
        <span class="kw">if </span>!rem.is_empty() {
            <span class="self">self</span>.buffer = counter.generate_block(<span class="kw-2">&amp;</span><span class="self">self</span>.nonce);
            <span class="self">self</span>.cipher.encrypt_block(<span class="kw-2">&amp;mut </span><span class="self">self</span>.buffer);
            xor(rem, <span class="kw-2">&amp;</span><span class="self">self</span>.buffer[..rem.len()]);
        }
        <span class="self">self</span>.buf_pos = rem.len() <span class="kw">as </span>u8;
        <span class="self">self</span>.counter = counter;
        <span class="prelude-val">Ok</span>(())
    }
}

<span class="kw">impl</span>&lt;B, F&gt; StreamCipherSeek <span class="kw">for </span>Ctr&lt;B, F&gt;
<span class="kw">where
    </span>B: BlockEncrypt,
    F: CtrFlavor&lt;B::BlockSize&gt;,
{
    <span class="kw">fn </span>try_current_pos&lt;T: SeekNum&gt;(<span class="kw-2">&amp;</span><span class="self">self</span>) -&gt; <span class="prelude-ty">Result</span>&lt;T, OverflowError&gt; {
        T::from_block_byte(<span class="self">self</span>.counter.to_backend(), <span class="self">self</span>.buf_pos, B::BlockSize::U8)
    }

    <span class="kw">fn </span>try_seek&lt;S: SeekNum&gt;(<span class="kw-2">&amp;mut </span><span class="self">self</span>, pos: S) -&gt; <span class="prelude-ty">Result</span>&lt;(), LoopError&gt; {
        <span class="kw">let </span>res: (F::Backend, u8) = pos.to_block_byte(B::BlockSize::U8)<span class="question-mark">?</span>;
        <span class="self">self</span>.counter = F::from_backend(res.<span class="number">0</span>);
        <span class="self">self</span>.buf_pos = res.<span class="number">1</span>;
        <span class="kw">if </span><span class="self">self</span>.buf_pos != <span class="number">0 </span>{
            <span class="kw">let </span><span class="kw-2">mut </span>block = <span class="self">self</span>.counter.generate_block(<span class="kw-2">&amp;</span><span class="self">self</span>.nonce);
            <span class="self">self</span>.cipher.encrypt_block(<span class="kw-2">&amp;mut </span>block);
            <span class="self">self</span>.buffer = block;
        }
        <span class="prelude-val">Ok</span>(())
    }
}

<span class="kw">impl</span>&lt;B, F&gt; fmt::Debug <span class="kw">for </span>Ctr&lt;B, F&gt;
<span class="kw">where
    </span>B: BlockEncrypt + fmt::Debug,
    F: CtrFlavor&lt;B::BlockSize&gt; + fmt::Debug,
{
    <span class="kw">fn </span>fmt(<span class="kw-2">&amp;</span><span class="self">self</span>, f: <span class="kw-2">&amp;mut </span>fmt::Formatter&lt;<span class="lifetime">'_</span>&gt;) -&gt; <span class="prelude-ty">Result</span>&lt;(), fmt::Error&gt; {
        <span class="macro">write!</span>(f, <span class="string">"Ctr-{:?}-{:?}"</span>, <span class="self">self</span>.counter, <span class="self">self</span>.cipher)
    }
}

<span class="attr">#[inline(always)]
</span><span class="kw">fn </span>xor(buf: <span class="kw-2">&amp;mut </span>[u8], key: <span class="kw-2">&amp;</span>[u8]) {
    <span class="macro">debug_assert_eq!</span>(buf.len(), key.len());
    <span class="kw">for </span>(a, b) <span class="kw">in </span>buf.iter_mut().zip(key) {
        <span class="kw-2">*</span>a ^= <span class="kw-2">*</span>b;
    }
}

<span class="attr">#[inline(always)]
</span><span class="kw">fn </span>to_slice&lt;C: BlockEncrypt&gt;(blocks: <span class="kw-2">&amp;</span>ParBlocks&lt;C&gt;) -&gt; <span class="kw-2">&amp;</span>[u8] {
    <span class="kw">let </span>blocks_len = C::BlockSize::to_usize() * C::ParBlocks::to_usize();
    <span class="kw">unsafe </span>{ core::slice::from_raw_parts(blocks.as_ptr() <span class="kw">as </span><span class="kw-2">*const </span>u8, blocks_len) }
}
</code></pre></div></section></main></body></html>