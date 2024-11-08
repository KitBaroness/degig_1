<!DOCTYPE html><html lang="en"><head><meta charset="utf-8"><meta name="viewport" content="width=device-width, initial-scale=1.0"><meta name="generator" content="rustdoc"><meta name="description" content="Source of the Rust file `/Users/jeyakumar/.cargo/registry/src/index.crates.io-6f17d22bba15001f/goblin-0.5.4/src/pe/section_table.rs`."><title>section_table.rs - source</title><script>if(window.location.protocol!=="file:")document.head.insertAdjacentHTML("beforeend","SourceSerif4-Regular-46f98efaafac5295.ttf.woff2,FiraSans-Regular-018c141bf0843ffd.woff2,FiraSans-Medium-8f9a781e4970d388.woff2,SourceCodePro-Regular-562dcc5011b6de7d.ttf.woff2,SourceCodePro-Semibold-d899c5a5c4aeb14a.ttf.woff2".split(",").map(f=>`<link rel="preload" as="font" type="font/woff2" crossorigin href="../../../static.files/${f}">`).join(""))</script><link rel="stylesheet" href="../../../static.files/normalize-76eba96aa4d2e634.css"><link rel="stylesheet" href="../../../static.files/rustdoc-081576b923113409.css"><meta name="rustdoc-vars" data-root-path="../../../" data-static-root-path="../../../static.files/" data-current-crate="goblin" data-themes="" data-resource-suffix="" data-rustdoc-version="1.79.0 (129f3b996 2024-06-10)" data-channel="1.79.0" data-search-js="search-bf21c90c8c1d92b1.js" data-settings-js="settings-4313503d2e1961c2.js" ><script src="../../../static.files/storage-e32f0c247825364d.js"></script><script defer src="../../../static.files/src-script-e66d777a5a92e9b2.js"></script><script defer src="../../../src-files.js"></script><script defer src="../../../static.files/main-20a3ad099b048cf2.js"></script><noscript><link rel="stylesheet" href="../../../static.files/noscript-09095024cf37855e.css"></noscript><link rel="alternate icon" type="image/png" href="../../../static.files/favicon-32x32-422f7d1d52889060.png"><link rel="icon" type="image/svg+xml" href="../../../static.files/favicon-2c020d218678b618.svg"></head><body class="rustdoc src"><!--[if lte IE 11]><div class="warning">This old browser is unsupported and will most likely display funky things.</div><![endif]--><nav class="sidebar"><div class="src-sidebar-title"><h2>Files</h2></div></nav><div class="sidebar-resizer"></div><main><nav class="sub"><form class="search-form"><span></span><div id="sidebar-button" tabindex="-1"><a href="../../../goblin/all.html" title="show sidebar"></a></div><input class="search-input" name="search" aria-label="Run search in the documentation" autocomplete="off" spellcheck="false" placeholder="Type ‘S’ or ‘/’ to search, ‘?’ for more options…" type="search"><div id="help-button" tabindex="-1"><a href="../../../help.html" title="help">?</a></div><div id="settings-menu" tabindex="-1"><a href="../../../settings.html" title="settings">Settings</a></div></form></nav><section id="main-content" class="content"><div class="example-wrap"><div data-nosnippet><pre class="src-line-numbers"><a href="#1" id="1">1</a>
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
<a href="#256" id="256">256</a>
<a href="#257" id="257">257</a>
<a href="#258" id="258">258</a>
<a href="#259" id="259">259</a>
<a href="#260" id="260">260</a>
<a href="#261" id="261">261</a>
<a href="#262" id="262">262</a>
<a href="#263" id="263">263</a>
<a href="#264" id="264">264</a>
<a href="#265" id="265">265</a>
<a href="#266" id="266">266</a>
<a href="#267" id="267">267</a>
<a href="#268" id="268">268</a>
<a href="#269" id="269">269</a>
<a href="#270" id="270">270</a>
<a href="#271" id="271">271</a>
<a href="#272" id="272">272</a>
<a href="#273" id="273">273</a>
<a href="#274" id="274">274</a>
<a href="#275" id="275">275</a>
<a href="#276" id="276">276</a>
<a href="#277" id="277">277</a>
<a href="#278" id="278">278</a>
<a href="#279" id="279">279</a>
</pre></div><pre class="rust"><code><span class="kw">use </span><span class="kw">crate</span>::error::{<span class="self">self</span>, Error};
<span class="kw">use </span><span class="kw">crate</span>::pe::relocation;
<span class="kw">use </span>alloc::string::{String, ToString};
<span class="kw">use </span>scroll::{ctx, Pread, Pwrite};

<span class="attr">#[repr(C)]
#[derive(Debug, PartialEq, Clone, Default)]
</span><span class="kw">pub struct </span>SectionTable {
    <span class="kw">pub </span>name: [u8; <span class="number">8</span>],
    <span class="kw">pub </span>real_name: <span class="prelude-ty">Option</span>&lt;String&gt;,
    <span class="kw">pub </span>virtual_size: u32,
    <span class="kw">pub </span>virtual_address: u32,
    <span class="kw">pub </span>size_of_raw_data: u32,
    <span class="kw">pub </span>pointer_to_raw_data: u32,
    <span class="kw">pub </span>pointer_to_relocations: u32,
    <span class="kw">pub </span>pointer_to_linenumbers: u32,
    <span class="kw">pub </span>number_of_relocations: u16,
    <span class="kw">pub </span>number_of_linenumbers: u16,
    <span class="kw">pub </span>characteristics: u32,
}

<span class="kw">pub const </span>SIZEOF_SECTION_TABLE: usize = <span class="number">8 </span>* <span class="number">5</span>;

<span class="comment">// Based on https://github.com/llvm-mirror/llvm/blob/af7b1832a03ab6486c42a40d21695b2c03b2d8a3/lib/Object/COFFObjectFile.cpp#L70
// Decodes a string table entry in base 64 (//AAAAAA). Expects string without
// prefixed slashes.
</span><span class="kw">fn </span>base64_decode_string_entry(s: <span class="kw-2">&amp;</span>str) -&gt; <span class="prelude-ty">Result</span>&lt;usize, ()&gt; {
    <span class="macro">assert!</span>(s.len() &lt;= <span class="number">6</span>, <span class="string">"String too long, possible overflow."</span>);

    <span class="kw">let </span><span class="kw-2">mut </span>val = <span class="number">0</span>;
    <span class="kw">for </span>c <span class="kw">in </span>s.bytes() {
        <span class="kw">let </span>v = <span class="kw">if </span><span class="string">b'A' </span>&lt;= c &amp;&amp; c &lt;= <span class="string">b'Z' </span>{
            <span class="comment">// 00..=25
            </span>c - <span class="string">b'A'
        </span>} <span class="kw">else if </span><span class="string">b'a' </span>&lt;= c &amp;&amp; c &lt;= <span class="string">b'z' </span>{
            <span class="comment">// 26..=51
            </span>c - <span class="string">b'a' </span>+ <span class="number">26
        </span>} <span class="kw">else if </span><span class="string">b'0' </span>&lt;= c &amp;&amp; c &lt;= <span class="string">b'9' </span>{
            <span class="comment">// 52..=61
            </span>c - <span class="string">b'0' </span>+ <span class="number">52
        </span>} <span class="kw">else if </span>c == <span class="string">b'+' </span>{
            <span class="comment">// 62
            </span><span class="number">62
        </span>} <span class="kw">else if </span>c == <span class="string">b'/' </span>{
            <span class="comment">// 63
            </span><span class="number">63
        </span>} <span class="kw">else </span>{
            <span class="kw">return </span><span class="prelude-val">Err</span>(());
        };
        val = val * <span class="number">64 </span>+ v <span class="kw">as </span>usize;
    }
    <span class="prelude-val">Ok</span>(val)
}

<span class="kw">impl </span>SectionTable {
    <span class="kw">pub fn </span>parse(
        bytes: <span class="kw-2">&amp;</span>[u8],
        offset: <span class="kw-2">&amp;mut </span>usize,
        string_table_offset: usize,
    ) -&gt; error::Result&lt;<span class="self">Self</span>&gt; {
        <span class="kw">let </span><span class="kw-2">mut </span>table = SectionTable::default();
        <span class="kw">let </span><span class="kw-2">mut </span>name = [<span class="number">0u8</span>; <span class="number">8</span>];
        name.copy_from_slice(bytes.gread_with(offset, <span class="number">8</span>)<span class="question-mark">?</span>);

        table.name = name;
        table.virtual_size = bytes.gread_with(offset, scroll::LE)<span class="question-mark">?</span>;
        table.virtual_address = bytes.gread_with(offset, scroll::LE)<span class="question-mark">?</span>;
        table.size_of_raw_data = bytes.gread_with(offset, scroll::LE)<span class="question-mark">?</span>;
        table.pointer_to_raw_data = bytes.gread_with(offset, scroll::LE)<span class="question-mark">?</span>;
        table.pointer_to_relocations = bytes.gread_with(offset, scroll::LE)<span class="question-mark">?</span>;
        table.pointer_to_linenumbers = bytes.gread_with(offset, scroll::LE)<span class="question-mark">?</span>;
        table.number_of_relocations = bytes.gread_with(offset, scroll::LE)<span class="question-mark">?</span>;
        table.number_of_linenumbers = bytes.gread_with(offset, scroll::LE)<span class="question-mark">?</span>;
        table.characteristics = bytes.gread_with(offset, scroll::LE)<span class="question-mark">?</span>;

        <span class="kw">if let </span><span class="prelude-val">Some</span>(idx) = table.name_offset()<span class="question-mark">? </span>{
            table.real_name = <span class="prelude-val">Some</span>(bytes.pread::&lt;<span class="kw-2">&amp;</span>str&gt;(string_table_offset + idx)<span class="question-mark">?</span>.to_string());
        }
        <span class="prelude-val">Ok</span>(table)
    }

    <span class="kw">pub fn </span>name_offset(<span class="kw-2">&amp;</span><span class="self">self</span>) -&gt; error::Result&lt;<span class="prelude-ty">Option</span>&lt;usize&gt;&gt; {
        <span class="comment">// Based on https://github.com/llvm-mirror/llvm/blob/af7b1832a03ab6486c42a40d21695b2c03b2d8a3/lib/Object/COFFObjectFile.cpp#L1054
        </span><span class="kw">if </span><span class="self">self</span>.name[<span class="number">0</span>] == <span class="string">b'/' </span>{
            <span class="kw">let </span>idx: usize = <span class="kw">if </span><span class="self">self</span>.name[<span class="number">1</span>] == <span class="string">b'/' </span>{
                <span class="kw">let </span>b64idx = <span class="self">self</span>.name.pread::&lt;<span class="kw-2">&amp;</span>str&gt;(<span class="number">2</span>)<span class="question-mark">?</span>;
                base64_decode_string_entry(b64idx).map_err(|<span class="kw">_</span>| {
                    Error::Malformed(<span class="macro">format!</span>(
                        <span class="string">"Invalid indirect section name //{}: base64 decoding failed"</span>,
                        b64idx
                    ))
                })<span class="question-mark">?
            </span>} <span class="kw">else </span>{
                <span class="kw">let </span>name = <span class="self">self</span>.name.pread::&lt;<span class="kw-2">&amp;</span>str&gt;(<span class="number">1</span>)<span class="question-mark">?</span>;
                name.parse().map_err(|err| {
                    Error::Malformed(<span class="macro">format!</span>(<span class="string">"Invalid indirect section name /{}: {}"</span>, name, err))
                })<span class="question-mark">?
            </span>};
            <span class="prelude-val">Ok</span>(<span class="prelude-val">Some</span>(idx))
        } <span class="kw">else </span>{
            <span class="prelude-val">Ok</span>(<span class="prelude-val">None</span>)
        }
    }

    <span class="attr">#[allow(clippy::useless_let_if_seq)]
    </span><span class="kw">pub fn </span>set_name_offset(<span class="kw-2">&amp;mut </span><span class="self">self</span>, <span class="kw-2">mut </span>idx: usize) -&gt; error::Result&lt;()&gt; {
        <span class="kw">if </span>idx &lt;= <span class="number">9_999_999 </span>{
            <span class="comment">// 10^7 - 1
            // write!(&amp;mut self.name[1..], "{}", idx) without using io::Write.
            // We write into a temporary since we calculate digits starting at the right.
            </span><span class="kw">let </span><span class="kw-2">mut </span>name = [<span class="number">0</span>; <span class="number">7</span>];
            <span class="kw">let </span><span class="kw-2">mut </span>len = <span class="number">0</span>;
            <span class="kw">if </span>idx == <span class="number">0 </span>{
                name[<span class="number">6</span>] = <span class="string">b'0'</span>;
                len = <span class="number">1</span>;
            } <span class="kw">else </span>{
                <span class="kw">while </span>idx != <span class="number">0 </span>{
                    <span class="kw">let </span>rem = (idx % <span class="number">10</span>) <span class="kw">as </span>u8;
                    idx /= <span class="number">10</span>;
                    name[<span class="number">6 </span>- len] = <span class="string">b'0' </span>+ rem;
                    len += <span class="number">1</span>;
                }
            }
            <span class="self">self</span>.name = [<span class="number">0</span>; <span class="number">8</span>];
            <span class="self">self</span>.name[<span class="number">0</span>] = <span class="string">b'/'</span>;
            <span class="self">self</span>.name[<span class="number">1</span>..][..len].copy_from_slice(<span class="kw-2">&amp;</span>name[<span class="number">7 </span>- len..]);
            <span class="prelude-val">Ok</span>(())
        } <span class="kw">else if </span>idx <span class="kw">as </span>u64 &lt;= <span class="number">0xfff_fff_fff </span>{
            <span class="comment">// 64^6 - 1
            </span><span class="self">self</span>.name[<span class="number">0</span>] = <span class="string">b'/'</span>;
            <span class="self">self</span>.name[<span class="number">1</span>] = <span class="string">b'/'</span>;
            <span class="kw">for </span>i <span class="kw">in </span><span class="number">0</span>..<span class="number">6 </span>{
                <span class="kw">let </span>rem = (idx % <span class="number">64</span>) <span class="kw">as </span>u8;
                idx /= <span class="number">64</span>;
                <span class="kw">let </span>c = <span class="kw">match </span>rem {
                    <span class="number">0</span>..=<span class="number">25 </span>=&gt; <span class="string">b'A' </span>+ rem,
                    <span class="number">26</span>..=<span class="number">51 </span>=&gt; <span class="string">b'a' </span>+ rem - <span class="number">26</span>,
                    <span class="number">52</span>..=<span class="number">61 </span>=&gt; <span class="string">b'0' </span>+ rem - <span class="number">52</span>,
                    <span class="number">62 </span>=&gt; <span class="string">b'+'</span>,
                    <span class="number">63 </span>=&gt; <span class="string">b'/'</span>,
                    <span class="kw">_ </span>=&gt; <span class="macro">unreachable!</span>(),
                };
                <span class="self">self</span>.name[<span class="number">7 </span>- i] = c;
            }
            <span class="prelude-val">Ok</span>(())
        } <span class="kw">else </span>{
            <span class="prelude-val">Err</span>(Error::Malformed(<span class="macro">format!</span>(
                <span class="string">"Invalid section name offset: {}"</span>,
                idx
            )))
        }
    }

    <span class="kw">pub fn </span>name(<span class="kw-2">&amp;</span><span class="self">self</span>) -&gt; error::Result&lt;<span class="kw-2">&amp;</span>str&gt; {
        <span class="kw">match </span><span class="self">self</span>.real_name.as_ref() {
            <span class="prelude-val">Some</span>(s) =&gt; <span class="prelude-val">Ok</span>(s),
            <span class="prelude-val">None </span>=&gt; <span class="prelude-val">Ok</span>(<span class="self">self</span>.name.pread(<span class="number">0</span>)<span class="question-mark">?</span>),
        }
    }

    <span class="kw">pub fn </span>relocations&lt;<span class="lifetime">'a</span>&gt;(<span class="kw-2">&amp;</span><span class="self">self</span>, bytes: <span class="kw-2">&amp;</span><span class="lifetime">'a </span>[u8]) -&gt; error::Result&lt;relocation::Relocations&lt;<span class="lifetime">'a</span>&gt;&gt; {
        <span class="kw">let </span>offset = <span class="self">self</span>.pointer_to_relocations <span class="kw">as </span>usize;
        <span class="kw">let </span>number = <span class="self">self</span>.number_of_relocations <span class="kw">as </span>usize;
        relocation::Relocations::parse(bytes, offset, number)
    }
}

<span class="kw">impl </span>ctx::SizeWith&lt;scroll::Endian&gt; <span class="kw">for </span>SectionTable {
    <span class="kw">fn </span>size_with(_ctx: <span class="kw-2">&amp;</span>scroll::Endian) -&gt; usize {
        SIZEOF_SECTION_TABLE
    }
}

<span class="kw">impl </span>ctx::TryIntoCtx&lt;scroll::Endian&gt; <span class="kw">for </span>SectionTable {
    <span class="kw">type </span>Error = error::Error;
    <span class="kw">fn </span>try_into_ctx(<span class="self">self</span>, bytes: <span class="kw-2">&amp;mut </span>[u8], ctx: scroll::Endian) -&gt; <span class="prelude-ty">Result</span>&lt;usize, <span class="self">Self</span>::Error&gt; {
        <span class="kw">let </span>offset = <span class="kw-2">&amp;mut </span><span class="number">0</span>;
        bytes.gwrite(<span class="kw-2">&amp;</span><span class="self">self</span>.name[..], offset)<span class="question-mark">?</span>;
        bytes.gwrite_with(<span class="self">self</span>.virtual_size, offset, ctx)<span class="question-mark">?</span>;
        bytes.gwrite_with(<span class="self">self</span>.virtual_address, offset, ctx)<span class="question-mark">?</span>;
        bytes.gwrite_with(<span class="self">self</span>.size_of_raw_data, offset, ctx)<span class="question-mark">?</span>;
        bytes.gwrite_with(<span class="self">self</span>.pointer_to_raw_data, offset, ctx)<span class="question-mark">?</span>;
        bytes.gwrite_with(<span class="self">self</span>.pointer_to_relocations, offset, ctx)<span class="question-mark">?</span>;
        bytes.gwrite_with(<span class="self">self</span>.pointer_to_linenumbers, offset, ctx)<span class="question-mark">?</span>;
        bytes.gwrite_with(<span class="self">self</span>.number_of_relocations, offset, ctx)<span class="question-mark">?</span>;
        bytes.gwrite_with(<span class="self">self</span>.number_of_linenumbers, offset, ctx)<span class="question-mark">?</span>;
        bytes.gwrite_with(<span class="self">self</span>.characteristics, offset, ctx)<span class="question-mark">?</span>;
        <span class="prelude-val">Ok</span>(SIZEOF_SECTION_TABLE)
    }
}

<span class="kw">impl </span>ctx::IntoCtx&lt;scroll::Endian&gt; <span class="kw">for </span>SectionTable {
    <span class="kw">fn </span>into_ctx(<span class="self">self</span>, bytes: <span class="kw-2">&amp;mut </span>[u8], ctx: scroll::Endian) {
        bytes.pwrite_with(<span class="self">self</span>, <span class="number">0</span>, ctx).unwrap();
    }
}

<span class="doccomment">/// The section should not be padded to the next boundary. This flag is obsolete and is replaced
/// by `IMAGE_SCN_ALIGN_1BYTES`. This is valid only for object files.
</span><span class="kw">pub const </span>IMAGE_SCN_TYPE_NO_PAD: u32 = <span class="number">0x0000_0008</span>;
<span class="doccomment">/// The section contains executable code.
</span><span class="kw">pub const </span>IMAGE_SCN_CNT_CODE: u32 = <span class="number">0x0000_0020</span>;
<span class="doccomment">/// The section contains initialized data.
</span><span class="kw">pub const </span>IMAGE_SCN_CNT_INITIALIZED_DATA: u32 = <span class="number">0x0000_0040</span>;
<span class="doccomment">///  The section contains uninitialized data.
</span><span class="kw">pub const </span>IMAGE_SCN_CNT_UNINITIALIZED_DATA: u32 = <span class="number">0x0000_0080</span>;
<span class="kw">pub const </span>IMAGE_SCN_LNK_OTHER: u32 = <span class="number">0x0000_0100</span>;
<span class="doccomment">/// The section contains comments or other information. The .drectve section has this type.
/// This is valid for object files only.
</span><span class="kw">pub const </span>IMAGE_SCN_LNK_INFO: u32 = <span class="number">0x0000_0200</span>;
<span class="doccomment">/// The section will not become part of the image. This is valid only for object files.
</span><span class="kw">pub const </span>IMAGE_SCN_LNK_REMOVE: u32 = <span class="number">0x0000_0800</span>;
<span class="doccomment">/// The section contains COMDAT data. This is valid only for object files.
</span><span class="kw">pub const </span>IMAGE_SCN_LNK_COMDAT: u32 = <span class="number">0x0000_1000</span>;
<span class="doccomment">/// The section contains data referenced through the global pointer (GP).
</span><span class="kw">pub const </span>IMAGE_SCN_GPREL: u32 = <span class="number">0x0000_8000</span>;
<span class="kw">pub const </span>IMAGE_SCN_MEM_PURGEABLE: u32 = <span class="number">0x0002_0000</span>;
<span class="kw">pub const </span>IMAGE_SCN_MEM_16BIT: u32 = <span class="number">0x0002_0000</span>;
<span class="kw">pub const </span>IMAGE_SCN_MEM_LOCKED: u32 = <span class="number">0x0004_0000</span>;
<span class="kw">pub const </span>IMAGE_SCN_MEM_PRELOAD: u32 = <span class="number">0x0008_0000</span>;

<span class="kw">pub const </span>IMAGE_SCN_ALIGN_1BYTES: u32 = <span class="number">0x0010_0000</span>;
<span class="kw">pub const </span>IMAGE_SCN_ALIGN_2BYTES: u32 = <span class="number">0x0020_0000</span>;
<span class="kw">pub const </span>IMAGE_SCN_ALIGN_4BYTES: u32 = <span class="number">0x0030_0000</span>;
<span class="kw">pub const </span>IMAGE_SCN_ALIGN_8BYTES: u32 = <span class="number">0x0040_0000</span>;
<span class="kw">pub const </span>IMAGE_SCN_ALIGN_16BYTES: u32 = <span class="number">0x0050_0000</span>;
<span class="kw">pub const </span>IMAGE_SCN_ALIGN_32BYTES: u32 = <span class="number">0x0060_0000</span>;
<span class="kw">pub const </span>IMAGE_SCN_ALIGN_64BYTES: u32 = <span class="number">0x0070_0000</span>;
<span class="kw">pub const </span>IMAGE_SCN_ALIGN_128BYTES: u32 = <span class="number">0x0080_0000</span>;
<span class="kw">pub const </span>IMAGE_SCN_ALIGN_256BYTES: u32 = <span class="number">0x0090_0000</span>;
<span class="kw">pub const </span>IMAGE_SCN_ALIGN_512BYTES: u32 = <span class="number">0x00A0_0000</span>;
<span class="kw">pub const </span>IMAGE_SCN_ALIGN_1024BYTES: u32 = <span class="number">0x00B0_0000</span>;
<span class="kw">pub const </span>IMAGE_SCN_ALIGN_2048BYTES: u32 = <span class="number">0x00C0_0000</span>;
<span class="kw">pub const </span>IMAGE_SCN_ALIGN_4096BYTES: u32 = <span class="number">0x00D0_0000</span>;
<span class="kw">pub const </span>IMAGE_SCN_ALIGN_8192BYTES: u32 = <span class="number">0x00E0_0000</span>;
<span class="kw">pub const </span>IMAGE_SCN_ALIGN_MASK: u32 = <span class="number">0x00F0_0000</span>;

<span class="doccomment">/// The section contains extended relocations.
</span><span class="kw">pub const </span>IMAGE_SCN_LNK_NRELOC_OVFL: u32 = <span class="number">0x0100_0000</span>;
<span class="doccomment">/// The section can be discarded as needed.
</span><span class="kw">pub const </span>IMAGE_SCN_MEM_DISCARDABLE: u32 = <span class="number">0x0200_0000</span>;
<span class="doccomment">/// The section cannot be cached.
</span><span class="kw">pub const </span>IMAGE_SCN_MEM_NOT_CACHED: u32 = <span class="number">0x0400_0000</span>;
<span class="doccomment">/// The section is not pageable.
</span><span class="kw">pub const </span>IMAGE_SCN_MEM_NOT_PAGED: u32 = <span class="number">0x0800_0000</span>;
<span class="doccomment">/// The section can be shared in memory.
</span><span class="kw">pub const </span>IMAGE_SCN_MEM_SHARED: u32 = <span class="number">0x1000_0000</span>;
<span class="doccomment">/// The section can be executed as code.
</span><span class="kw">pub const </span>IMAGE_SCN_MEM_EXECUTE: u32 = <span class="number">0x2000_0000</span>;
<span class="doccomment">/// The section can be read.
</span><span class="kw">pub const </span>IMAGE_SCN_MEM_READ: u32 = <span class="number">0x4000_0000</span>;
<span class="doccomment">/// The section can be written to.
</span><span class="kw">pub const </span>IMAGE_SCN_MEM_WRITE: u32 = <span class="number">0x8000_0000</span>;

<span class="attr">#[cfg(test)]
</span><span class="kw">mod </span>tests {
    <span class="kw">use super</span>::<span class="kw-2">*</span>;

    <span class="attr">#[test]
    </span><span class="kw">fn </span>set_name_offset() {
        <span class="kw">let </span><span class="kw-2">mut </span>section = SectionTable::default();
        <span class="kw">for </span><span class="kw-2">&amp;</span>(offset, name) <span class="kw">in </span>[
            (<span class="number">0usize</span>, <span class="string">b"/0\0\0\0\0\0\0"</span>),
            (<span class="number">1</span>, <span class="string">b"/1\0\0\0\0\0\0"</span>),
            (<span class="number">9_999_999</span>, <span class="string">b"/9999999"</span>),
            (<span class="number">10_000_000</span>, <span class="string">b"//AAmJaA"</span>),
            <span class="attr">#[cfg(target_pointer_width = <span class="string">"64"</span>)]
            </span>(<span class="number">0xfff_fff_fff</span>, <span class="string">b"////////"</span>),
        ]
        .iter()
        {
            section.set_name_offset(offset).unwrap();
            <span class="macro">assert_eq!</span>(<span class="kw-2">&amp;</span>section.name, name);
            <span class="macro">assert_eq!</span>(section.name_offset().unwrap(), <span class="prelude-val">Some</span>(offset));
        }
        <span class="attr">#[cfg(target_pointer_width = <span class="string">"64"</span>)]
        </span><span class="macro">assert!</span>(section.set_name_offset(<span class="number">0x1_000_000_000</span>).is_err());
    }
}
</code></pre></div></section></main></body></html>