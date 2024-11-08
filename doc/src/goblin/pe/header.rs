<!DOCTYPE html><html lang="en"><head><meta charset="utf-8"><meta name="viewport" content="width=device-width, initial-scale=1.0"><meta name="generator" content="rustdoc"><meta name="description" content="Source of the Rust file `/Users/jeyakumar/.cargo/registry/src/index.crates.io-6f17d22bba15001f/goblin-0.5.4/src/pe/header.rs`."><title>header.rs - source</title><script>if(window.location.protocol!=="file:")document.head.insertAdjacentHTML("beforeend","SourceSerif4-Regular-46f98efaafac5295.ttf.woff2,FiraSans-Regular-018c141bf0843ffd.woff2,FiraSans-Medium-8f9a781e4970d388.woff2,SourceCodePro-Regular-562dcc5011b6de7d.ttf.woff2,SourceCodePro-Semibold-d899c5a5c4aeb14a.ttf.woff2".split(",").map(f=>`<link rel="preload" as="font" type="font/woff2" crossorigin href="../../../static.files/${f}">`).join(""))</script><link rel="stylesheet" href="../../../static.files/normalize-76eba96aa4d2e634.css"><link rel="stylesheet" href="../../../static.files/rustdoc-081576b923113409.css"><meta name="rustdoc-vars" data-root-path="../../../" data-static-root-path="../../../static.files/" data-current-crate="goblin" data-themes="" data-resource-suffix="" data-rustdoc-version="1.79.0 (129f3b996 2024-06-10)" data-channel="1.79.0" data-search-js="search-bf21c90c8c1d92b1.js" data-settings-js="settings-4313503d2e1961c2.js" ><script src="../../../static.files/storage-e32f0c247825364d.js"></script><script defer src="../../../static.files/src-script-e66d777a5a92e9b2.js"></script><script defer src="../../../src-files.js"></script><script defer src="../../../static.files/main-20a3ad099b048cf2.js"></script><noscript><link rel="stylesheet" href="../../../static.files/noscript-09095024cf37855e.css"></noscript><link rel="alternate icon" type="image/png" href="../../../static.files/favicon-32x32-422f7d1d52889060.png"><link rel="icon" type="image/svg+xml" href="../../../static.files/favicon-2c020d218678b618.svg"></head><body class="rustdoc src"><!--[if lte IE 11]><div class="warning">This old browser is unsupported and will most likely display funky things.</div><![endif]--><nav class="sidebar"><div class="src-sidebar-title"><h2>Files</h2></div></nav><div class="sidebar-resizer"></div><main><nav class="sub"><form class="search-form"><span></span><div id="sidebar-button" tabindex="-1"><a href="../../../goblin/all.html" title="show sidebar"></a></div><input class="search-input" name="search" aria-label="Run search in the documentation" autocomplete="off" spellcheck="false" placeholder="Type ‘S’ or ‘/’ to search, ‘?’ for more options…" type="search"><div id="help-button" tabindex="-1"><a href="../../../help.html" title="help">?</a></div><div id="settings-menu" tabindex="-1"><a href="../../../settings.html" title="settings">Settings</a></div></form></nav><section id="main-content" class="content"><div class="example-wrap"><div data-nosnippet><pre class="src-line-numbers"><a href="#1" id="1">1</a>
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
</pre></div><pre class="rust"><code><span class="kw">use </span><span class="kw">crate</span>::error;
<span class="kw">use </span><span class="kw">crate</span>::pe::{optional_header, section_table, symbol};
<span class="kw">use </span><span class="kw">crate</span>::strtab;
<span class="kw">use </span>alloc::vec::Vec;
<span class="kw">use </span>log::debug;
<span class="kw">use </span>scroll::{IOread, IOwrite, Pread, Pwrite, SizeWith};

<span class="doccomment">/// DOS header present in all PE binaries
</span><span class="attr">#[repr(C)]
#[derive(Debug, PartialEq, Copy, Clone, Default)]
</span><span class="kw">pub struct </span>DosHeader {
    <span class="doccomment">/// Magic number: 5a4d
    </span><span class="kw">pub </span>signature: u16,
    <span class="doccomment">/// Pointer to PE header, always at offset 0x3c
    </span><span class="kw">pub </span>pe_pointer: u32,
}

<span class="kw">pub const </span>DOS_MAGIC: u16 = <span class="number">0x5a4d</span>;
<span class="kw">pub const </span>PE_POINTER_OFFSET: u32 = <span class="number">0x3c</span>;

<span class="kw">impl </span>DosHeader {
    <span class="kw">pub fn </span>parse(bytes: <span class="kw-2">&amp;</span>[u8]) -&gt; error::Result&lt;<span class="self">Self</span>&gt; {
        <span class="kw">let </span>signature = bytes.pread_with(<span class="number">0</span>, scroll::LE).map_err(|<span class="kw">_</span>| {
            error::Error::Malformed(<span class="macro">format!</span>(<span class="string">"cannot parse DOS signature (offset {:#x})"</span>, <span class="number">0</span>))
        })<span class="question-mark">?</span>;
        <span class="kw">if </span>signature != DOS_MAGIC {
            <span class="kw">return </span><span class="prelude-val">Err</span>(error::Error::Malformed(<span class="macro">format!</span>(
                <span class="string">"DOS header is malformed (signature {:#x})"</span>,
                signature
            )));
        }
        <span class="kw">let </span>pe_pointer = bytes
            .pread_with(PE_POINTER_OFFSET <span class="kw">as </span>usize, scroll::LE)
            .map_err(|<span class="kw">_</span>| {
                error::Error::Malformed(<span class="macro">format!</span>(
                    <span class="string">"cannot parse PE header pointer (offset {:#x})"</span>,
                    PE_POINTER_OFFSET
                ))
            })<span class="question-mark">?</span>;
        <span class="kw">let </span>pe_signature: u32 =
            bytes
                .pread_with(pe_pointer <span class="kw">as </span>usize, scroll::LE)
                .map_err(|<span class="kw">_</span>| {
                    error::Error::Malformed(<span class="macro">format!</span>(
                        <span class="string">"cannot parse PE header signature (offset {:#x})"</span>,
                        pe_pointer
                    ))
                })<span class="question-mark">?</span>;
        <span class="kw">if </span>pe_signature != PE_MAGIC {
            <span class="kw">return </span><span class="prelude-val">Err</span>(error::Error::Malformed(<span class="macro">format!</span>(
                <span class="string">"PE header is malformed (signature {:#x})"</span>,
                pe_signature
            )));
        }
        <span class="prelude-val">Ok</span>(DosHeader {
            signature,
            pe_pointer,
        })
    }
}

<span class="doccomment">/// COFF Header
</span><span class="attr">#[repr(C)]
#[derive(Debug, PartialEq, Copy, Clone, Default, Pread, Pwrite, IOread, IOwrite, SizeWith)]
</span><span class="kw">pub struct </span>CoffHeader {
    <span class="doccomment">/// The machine type
    </span><span class="kw">pub </span>machine: u16,
    <span class="kw">pub </span>number_of_sections: u16,
    <span class="kw">pub </span>time_date_stamp: u32,
    <span class="kw">pub </span>pointer_to_symbol_table: u32,
    <span class="kw">pub </span>number_of_symbol_table: u32,
    <span class="kw">pub </span>size_of_optional_header: u16,
    <span class="kw">pub </span>characteristics: u16,
}

<span class="kw">pub const </span>SIZEOF_COFF_HEADER: usize = <span class="number">20</span>;
<span class="doccomment">/// PE\0\0, little endian
</span><span class="kw">pub const </span>PE_MAGIC: u32 = <span class="number">0x0000_4550</span>;
<span class="kw">pub const </span>SIZEOF_PE_MAGIC: usize = <span class="number">4</span>;
<span class="doccomment">/// The contents of this field are assumed to be applicable to any machine type
</span><span class="kw">pub const </span>COFF_MACHINE_UNKNOWN: u16 = <span class="number">0x0</span>;
<span class="doccomment">/// Matsushita AM33
</span><span class="kw">pub const </span>COFF_MACHINE_AM33: u16 = <span class="number">0x1d3</span>;
<span class="doccomment">/// x64
</span><span class="kw">pub const </span>COFF_MACHINE_X86_64: u16 = <span class="number">0x8664</span>;
<span class="doccomment">/// ARM little endian
</span><span class="kw">pub const </span>COFF_MACHINE_ARM: u16 = <span class="number">0x1c0</span>;
<span class="doccomment">/// ARM64 little endian
</span><span class="kw">pub const </span>COFF_MACHINE_ARM64: u16 = <span class="number">0xaa64</span>;
<span class="doccomment">/// ARM Thumb-2 little endian
</span><span class="kw">pub const </span>COFF_MACHINE_ARMNT: u16 = <span class="number">0x1c4</span>;
<span class="doccomment">/// EFI byte code
</span><span class="kw">pub const </span>COFF_MACHINE_EBC: u16 = <span class="number">0xebc</span>;
<span class="doccomment">/// Intel 386 or later processors and compatible processors
</span><span class="kw">pub const </span>COFF_MACHINE_X86: u16 = <span class="number">0x14c</span>;
<span class="doccomment">/// Intel Itanium processor family
</span><span class="kw">pub const </span>COFF_MACHINE_IA64: u16 = <span class="number">0x200</span>;
<span class="doccomment">/// Mitsubishi M32R little endian
</span><span class="kw">pub const </span>COFF_MACHINE_M32R: u16 = <span class="number">0x9041</span>;
<span class="doccomment">/// MIPS16
</span><span class="kw">pub const </span>COFF_MACHINE_MIPS16: u16 = <span class="number">0x266</span>;
<span class="doccomment">/// MIPS with FPU
</span><span class="kw">pub const </span>COFF_MACHINE_MIPSFPU: u16 = <span class="number">0x366</span>;
<span class="doccomment">/// MIPS16 with FPU
</span><span class="kw">pub const </span>COFF_MACHINE_MIPSFPU16: u16 = <span class="number">0x466</span>;
<span class="doccomment">/// Power PC little endian
</span><span class="kw">pub const </span>COFF_MACHINE_POWERPC: u16 = <span class="number">0x1f0</span>;
<span class="doccomment">/// Power PC with floating point support
</span><span class="kw">pub const </span>COFF_MACHINE_POWERPCFP: u16 = <span class="number">0x1f1</span>;
<span class="doccomment">/// MIPS little endian
</span><span class="kw">pub const </span>COFF_MACHINE_R4000: u16 = <span class="number">0x166</span>;
<span class="doccomment">/// RISC-V 32-bit address space
</span><span class="kw">pub const </span>COFF_MACHINE_RISCV32: u16 = <span class="number">0x5032</span>;
<span class="doccomment">/// RISC-V 64-bit address space
</span><span class="kw">pub const </span>COFF_MACHINE_RISCV64: u16 = <span class="number">0x5064</span>;
<span class="doccomment">/// RISC-V 128-bit address space
</span><span class="kw">pub const </span>COFF_MACHINE_RISCV128: u16 = <span class="number">0x5128</span>;
<span class="doccomment">/// Hitachi SH3
</span><span class="kw">pub const </span>COFF_MACHINE_SH3: u16 = <span class="number">0x1a2</span>;
<span class="doccomment">/// Hitachi SH3 DSP
</span><span class="kw">pub const </span>COFF_MACHINE_SH3DSP: u16 = <span class="number">0x1a3</span>;
<span class="doccomment">/// Hitachi SH4
</span><span class="kw">pub const </span>COFF_MACHINE_SH4: u16 = <span class="number">0x1a6</span>;
<span class="doccomment">/// Hitachi SH5
</span><span class="kw">pub const </span>COFF_MACHINE_SH5: u16 = <span class="number">0x1a8</span>;
<span class="doccomment">/// Thumb
</span><span class="kw">pub const </span>COFF_MACHINE_THUMB: u16 = <span class="number">0x1c2</span>;
<span class="doccomment">/// MIPS little-endian WCE v2
</span><span class="kw">pub const </span>COFF_MACHINE_WCEMIPSV2: u16 = <span class="number">0x169</span>;

<span class="kw">impl </span>CoffHeader {
    <span class="kw">pub fn </span>parse(bytes: <span class="kw-2">&amp;</span>[u8], offset: <span class="kw-2">&amp;mut </span>usize) -&gt; error::Result&lt;<span class="self">Self</span>&gt; {
        <span class="prelude-val">Ok</span>(bytes.gread_with(offset, scroll::LE)<span class="question-mark">?</span>)
    }

    <span class="doccomment">/// Parse the COFF section headers.
    ///
    /// For COFF, these immediately follow the COFF header. For PE, these immediately follow the
    /// optional header.
    </span><span class="kw">pub fn </span>sections(
        <span class="kw-2">&amp;</span><span class="self">self</span>,
        bytes: <span class="kw-2">&amp;</span>[u8],
        offset: <span class="kw-2">&amp;mut </span>usize,
    ) -&gt; error::Result&lt;Vec&lt;section_table::SectionTable&gt;&gt; {
        <span class="kw">let </span>nsections = <span class="self">self</span>.number_of_sections <span class="kw">as </span>usize;

        <span class="comment">// a section table is at least 40 bytes
        </span><span class="kw">if </span>nsections &gt; bytes.len() / <span class="number">40 </span>{
            <span class="kw">return </span><span class="prelude-val">Err</span>(error::Error::BufferTooShort(nsections, <span class="string">"sections"</span>));
        }

        <span class="kw">let </span><span class="kw-2">mut </span>sections = Vec::with_capacity(nsections);
        <span class="comment">// Note that if we are handling a BigCoff, the size of the symbol will be different!
        </span><span class="kw">let </span>string_table_offset = <span class="self">self</span>.pointer_to_symbol_table <span class="kw">as </span>usize
            + symbol::SymbolTable::size(<span class="self">self</span>.number_of_symbol_table <span class="kw">as </span>usize);
        <span class="kw">for </span>i <span class="kw">in </span><span class="number">0</span>..nsections {
            <span class="kw">let </span>section =
                section_table::SectionTable::parse(bytes, offset, string_table_offset <span class="kw">as </span>usize)<span class="question-mark">?</span>;
            <span class="macro">debug!</span>(<span class="string">"({}) {:#?}"</span>, i, section);
            sections.push(section);
        }
        <span class="prelude-val">Ok</span>(sections)
    }

    <span class="doccomment">/// Return the COFF symbol table.
    </span><span class="kw">pub fn </span>symbols&lt;<span class="lifetime">'a</span>&gt;(<span class="kw-2">&amp;</span><span class="self">self</span>, bytes: <span class="kw-2">&amp;</span><span class="lifetime">'a </span>[u8]) -&gt; error::Result&lt;symbol::SymbolTable&lt;<span class="lifetime">'a</span>&gt;&gt; {
        <span class="kw">let </span>offset = <span class="self">self</span>.pointer_to_symbol_table <span class="kw">as </span>usize;
        <span class="kw">let </span>number = <span class="self">self</span>.number_of_symbol_table <span class="kw">as </span>usize;
        symbol::SymbolTable::parse(bytes, offset, number)
    }

    <span class="doccomment">/// Return the COFF string table.
    </span><span class="kw">pub fn </span>strings&lt;<span class="lifetime">'a</span>&gt;(<span class="kw-2">&amp;</span><span class="self">self</span>, bytes: <span class="kw-2">&amp;</span><span class="lifetime">'a </span>[u8]) -&gt; error::Result&lt;strtab::Strtab&lt;<span class="lifetime">'a</span>&gt;&gt; {
        <span class="kw">let </span><span class="kw-2">mut </span>offset = <span class="self">self</span>.pointer_to_symbol_table <span class="kw">as </span>usize
            + symbol::SymbolTable::size(<span class="self">self</span>.number_of_symbol_table <span class="kw">as </span>usize);

        <span class="kw">let </span>length_field_size = core::mem::size_of::&lt;u32&gt;();
        <span class="kw">let </span>length = bytes.pread_with::&lt;u32&gt;(offset, scroll::LE)<span class="question-mark">? </span><span class="kw">as </span>usize - length_field_size;

        <span class="comment">// The offset needs to be advanced in order to read the strings.
        </span>offset += length_field_size;

        <span class="prelude-val">Ok</span>(strtab::Strtab::parse(bytes, offset, length, <span class="number">0</span>)<span class="question-mark">?</span>)
    }
}

<span class="attr">#[derive(Debug, PartialEq, Copy, Clone, Default)]
</span><span class="kw">pub struct </span>Header {
    <span class="kw">pub </span>dos_header: DosHeader,
    <span class="doccomment">/// PE Magic: PE\0\0, little endian
    </span><span class="kw">pub </span>signature: u32,
    <span class="kw">pub </span>coff_header: CoffHeader,
    <span class="kw">pub </span>optional_header: <span class="prelude-ty">Option</span>&lt;optional_header::OptionalHeader&gt;,
}

<span class="kw">impl </span>Header {
    <span class="kw">pub fn </span>parse(bytes: <span class="kw-2">&amp;</span>[u8]) -&gt; error::Result&lt;<span class="self">Self</span>&gt; {
        <span class="kw">let </span>dos_header = DosHeader::parse(<span class="kw-2">&amp;</span>bytes)<span class="question-mark">?</span>;
        <span class="kw">let </span><span class="kw-2">mut </span>offset = dos_header.pe_pointer <span class="kw">as </span>usize;
        <span class="kw">let </span>signature = bytes.gread_with(<span class="kw-2">&amp;mut </span>offset, scroll::LE).map_err(|<span class="kw">_</span>| {
            error::Error::Malformed(<span class="macro">format!</span>(<span class="string">"cannot parse PE signature (offset {:#x})"</span>, offset))
        })<span class="question-mark">?</span>;
        <span class="kw">let </span>coff_header = CoffHeader::parse(<span class="kw-2">&amp;</span>bytes, <span class="kw-2">&amp;mut </span>offset)<span class="question-mark">?</span>;
        <span class="kw">let </span>optional_header = <span class="kw">if </span>coff_header.size_of_optional_header &gt; <span class="number">0 </span>{
            <span class="prelude-val">Some</span>(bytes.pread::&lt;optional_header::OptionalHeader&gt;(offset)<span class="question-mark">?</span>)
        } <span class="kw">else </span>{
            <span class="prelude-val">None
        </span>};
        <span class="prelude-val">Ok</span>(Header {
            dos_header,
            signature,
            coff_header,
            optional_header,
        })
    }
}

<span class="attr">#[cfg(test)]
</span><span class="kw">mod </span>tests {
    <span class="kw">use super</span>::{Header, COFF_MACHINE_X86, DOS_MAGIC, PE_MAGIC};

    <span class="kw">const </span>CRSS_HEADER: [u8; <span class="number">688</span>] = [
        <span class="number">0x4d</span>, <span class="number">0x5a</span>, <span class="number">0x90</span>, <span class="number">0x00</span>, <span class="number">0x03</span>, <span class="number">0x00</span>, <span class="number">0x00</span>, <span class="number">0x00</span>, <span class="number">0x04</span>, <span class="number">0x00</span>, <span class="number">0x00</span>, <span class="number">0x00</span>, <span class="number">0xff</span>, <span class="number">0xff</span>, <span class="number">0x00</span>,
        <span class="number">0x00</span>, <span class="number">0xb8</span>, <span class="number">0x00</span>, <span class="number">0x00</span>, <span class="number">0x00</span>, <span class="number">0x00</span>, <span class="number">0x00</span>, <span class="number">0x00</span>, <span class="number">0x00</span>, <span class="number">0x40</span>, <span class="number">0x00</span>, <span class="number">0x00</span>, <span class="number">0x00</span>, <span class="number">0x00</span>, <span class="number">0x00</span>,
        <span class="number">0x00</span>, <span class="number">0x00</span>, <span class="number">0x00</span>, <span class="number">0x00</span>, <span class="number">0x00</span>, <span class="number">0x00</span>, <span class="number">0x00</span>, <span class="number">0x00</span>, <span class="number">0x00</span>, <span class="number">0x00</span>, <span class="number">0x00</span>, <span class="number">0x00</span>, <span class="number">0x00</span>, <span class="number">0x00</span>, <span class="number">0x00</span>,
        <span class="number">0x00</span>, <span class="number">0x00</span>, <span class="number">0x00</span>, <span class="number">0x00</span>, <span class="number">0x00</span>, <span class="number">0x00</span>, <span class="number">0x00</span>, <span class="number">0x00</span>, <span class="number">0x00</span>, <span class="number">0x00</span>, <span class="number">0x00</span>, <span class="number">0x00</span>, <span class="number">0x00</span>, <span class="number">0x00</span>, <span class="number">0x00</span>,
        <span class="number">0xd0</span>, <span class="number">0x00</span>, <span class="number">0x00</span>, <span class="number">0x00</span>, <span class="number">0x0e</span>, <span class="number">0x1f</span>, <span class="number">0xba</span>, <span class="number">0x0e</span>, <span class="number">0x00</span>, <span class="number">0xb4</span>, <span class="number">0x09</span>, <span class="number">0xcd</span>, <span class="number">0x21</span>, <span class="number">0xb8</span>, <span class="number">0x01</span>,
        <span class="number">0x4c</span>, <span class="number">0xcd</span>, <span class="number">0x21</span>, <span class="number">0x54</span>, <span class="number">0x68</span>, <span class="number">0x69</span>, <span class="number">0x73</span>, <span class="number">0x20</span>, <span class="number">0x70</span>, <span class="number">0x72</span>, <span class="number">0x6f</span>, <span class="number">0x67</span>, <span class="number">0x72</span>, <span class="number">0x61</span>, <span class="number">0x6d</span>,
        <span class="number">0x20</span>, <span class="number">0x63</span>, <span class="number">0x61</span>, <span class="number">0x6e</span>, <span class="number">0x6e</span>, <span class="number">0x6f</span>, <span class="number">0x74</span>, <span class="number">0x20</span>, <span class="number">0x62</span>, <span class="number">0x65</span>, <span class="number">0x20</span>, <span class="number">0x72</span>, <span class="number">0x75</span>, <span class="number">0x6e</span>, <span class="number">0x20</span>,
        <span class="number">0x69</span>, <span class="number">0x6e</span>, <span class="number">0x20</span>, <span class="number">0x44</span>, <span class="number">0x4f</span>, <span class="number">0x53</span>, <span class="number">0x20</span>, <span class="number">0x6d</span>, <span class="number">0x6f</span>, <span class="number">0x64</span>, <span class="number">0x65</span>, <span class="number">0x2e</span>, <span class="number">0x0d</span>, <span class="number">0x0d</span>, <span class="number">0x0a</span>,
        <span class="number">0x24</span>, <span class="number">0x00</span>, <span class="number">0x00</span>, <span class="number">0x00</span>, <span class="number">0x00</span>, <span class="number">0x00</span>, <span class="number">0x00</span>, <span class="number">0x00</span>, <span class="number">0xaa</span>, <span class="number">0x4a</span>, <span class="number">0xc3</span>, <span class="number">0xeb</span>, <span class="number">0xee</span>, <span class="number">0x2b</span>, <span class="number">0xad</span>,
        <span class="number">0xb8</span>, <span class="number">0xee</span>, <span class="number">0x2b</span>, <span class="number">0xad</span>, <span class="number">0xb8</span>, <span class="number">0xee</span>, <span class="number">0x2b</span>, <span class="number">0xad</span>, <span class="number">0xb8</span>, <span class="number">0xee</span>, <span class="number">0x2b</span>, <span class="number">0xac</span>, <span class="number">0xb8</span>, <span class="number">0xfe</span>, <span class="number">0x2b</span>,
        <span class="number">0xad</span>, <span class="number">0xb8</span>, <span class="number">0x33</span>, <span class="number">0xd4</span>, <span class="number">0x66</span>, <span class="number">0xb8</span>, <span class="number">0xeb</span>, <span class="number">0x2b</span>, <span class="number">0xad</span>, <span class="number">0xb8</span>, <span class="number">0x33</span>, <span class="number">0xd4</span>, <span class="number">0x63</span>, <span class="number">0xb8</span>, <span class="number">0xea</span>,
        <span class="number">0x2b</span>, <span class="number">0xad</span>, <span class="number">0xb8</span>, <span class="number">0x33</span>, <span class="number">0xd4</span>, <span class="number">0x7a</span>, <span class="number">0xb8</span>, <span class="number">0xed</span>, <span class="number">0x2b</span>, <span class="number">0xad</span>, <span class="number">0xb8</span>, <span class="number">0x33</span>, <span class="number">0xd4</span>, <span class="number">0x64</span>, <span class="number">0xb8</span>,
        <span class="number">0xef</span>, <span class="number">0x2b</span>, <span class="number">0xad</span>, <span class="number">0xb8</span>, <span class="number">0x33</span>, <span class="number">0xd4</span>, <span class="number">0x61</span>, <span class="number">0xb8</span>, <span class="number">0xef</span>, <span class="number">0x2b</span>, <span class="number">0xad</span>, <span class="number">0xb8</span>, <span class="number">0x52</span>, <span class="number">0x69</span>, <span class="number">0x63</span>,
        <span class="number">0x68</span>, <span class="number">0xee</span>, <span class="number">0x2b</span>, <span class="number">0xad</span>, <span class="number">0xb8</span>, <span class="number">0x00</span>, <span class="number">0x00</span>, <span class="number">0x00</span>, <span class="number">0x00</span>, <span class="number">0x00</span>, <span class="number">0x00</span>, <span class="number">0x00</span>, <span class="number">0x00</span>, <span class="number">0x50</span>, <span class="number">0x45</span>,
        <span class="number">0x00</span>, <span class="number">0x00</span>, <span class="number">0x4c</span>, <span class="number">0x01</span>, <span class="number">0x05</span>, <span class="number">0x00</span>, <span class="number">0xd9</span>, <span class="number">0x8f</span>, <span class="number">0x15</span>, <span class="number">0x52</span>, <span class="number">0x00</span>, <span class="number">0x00</span>, <span class="number">0x00</span>, <span class="number">0x00</span>, <span class="number">0x00</span>,
        <span class="number">0x00</span>, <span class="number">0x00</span>, <span class="number">0x00</span>, <span class="number">0xe0</span>, <span class="number">0x00</span>, <span class="number">0x02</span>, <span class="number">0x01</span>, <span class="number">0x0b</span>, <span class="number">0x01</span>, <span class="number">0x0b</span>, <span class="number">0x00</span>, <span class="number">0x00</span>, <span class="number">0x08</span>, <span class="number">0x00</span>, <span class="number">0x00</span>,
        <span class="number">0x00</span>, <span class="number">0x10</span>, <span class="number">0x00</span>, <span class="number">0x00</span>, <span class="number">0x00</span>, <span class="number">0x00</span>, <span class="number">0x00</span>, <span class="number">0x00</span>, <span class="number">0x10</span>, <span class="number">0x11</span>, <span class="number">0x00</span>, <span class="number">0x00</span>, <span class="number">0x00</span>, <span class="number">0x10</span>, <span class="number">0x00</span>,
        <span class="number">0x00</span>, <span class="number">0x00</span>, <span class="number">0x20</span>, <span class="number">0x00</span>, <span class="number">0x00</span>, <span class="number">0x00</span>, <span class="number">0x00</span>, <span class="number">0x40</span>, <span class="number">0x00</span>, <span class="number">0x00</span>, <span class="number">0x10</span>, <span class="number">0x00</span>, <span class="number">0x00</span>, <span class="number">0x00</span>, <span class="number">0x02</span>,
        <span class="number">0x00</span>, <span class="number">0x00</span>, <span class="number">0x06</span>, <span class="number">0x00</span>, <span class="number">0x03</span>, <span class="number">0x00</span>, <span class="number">0x06</span>, <span class="number">0x00</span>, <span class="number">0x03</span>, <span class="number">0x00</span>, <span class="number">0x06</span>, <span class="number">0x00</span>, <span class="number">0x03</span>, <span class="number">0x00</span>, <span class="number">0x00</span>,
        <span class="number">0x00</span>, <span class="number">0x00</span>, <span class="number">0x00</span>, <span class="number">0x00</span>, <span class="number">0x60</span>, <span class="number">0x00</span>, <span class="number">0x00</span>, <span class="number">0x00</span>, <span class="number">0x04</span>, <span class="number">0x00</span>, <span class="number">0x00</span>, <span class="number">0xe4</span>, <span class="number">0xab</span>, <span class="number">0x00</span>, <span class="number">0x00</span>,
        <span class="number">0x01</span>, <span class="number">0x00</span>, <span class="number">0x40</span>, <span class="number">0x05</span>, <span class="number">0x00</span>, <span class="number">0x00</span>, <span class="number">0x04</span>, <span class="number">0x00</span>, <span class="number">0x00</span>, <span class="number">0x30</span>, <span class="number">0x00</span>, <span class="number">0x00</span>, <span class="number">0x00</span>, <span class="number">0x00</span>, <span class="number">0x10</span>,
        <span class="number">0x00</span>, <span class="number">0x00</span>, <span class="number">0x10</span>, <span class="number">0x00</span>, <span class="number">0x00</span>, <span class="number">0x00</span>, <span class="number">0x00</span>, <span class="number">0x00</span>, <span class="number">0x00</span>, <span class="number">0x10</span>, <span class="number">0x00</span>, <span class="number">0x00</span>, <span class="number">0x00</span>, <span class="number">0x00</span>, <span class="number">0x00</span>,
        <span class="number">0x00</span>, <span class="number">0x00</span>, <span class="number">0x00</span>, <span class="number">0x00</span>, <span class="number">0x00</span>, <span class="number">0x00</span>, <span class="number">0x3c</span>, <span class="number">0x30</span>, <span class="number">0x00</span>, <span class="number">0x00</span>, <span class="number">0x3c</span>, <span class="number">0x00</span>, <span class="number">0x00</span>, <span class="number">0x00</span>, <span class="number">0x00</span>,
        <span class="number">0x40</span>, <span class="number">0x00</span>, <span class="number">0x00</span>, <span class="number">0x00</span>, <span class="number">0x08</span>, <span class="number">0x00</span>, <span class="number">0x00</span>, <span class="number">0x00</span>, <span class="number">0x00</span>, <span class="number">0x00</span>, <span class="number">0x00</span>, <span class="number">0x00</span>, <span class="number">0x00</span>, <span class="number">0x00</span>, <span class="number">0x00</span>,
        <span class="number">0x00</span>, <span class="number">0x1a</span>, <span class="number">0x00</span>, <span class="number">0x00</span>, <span class="number">0xb8</span>, <span class="number">0x22</span>, <span class="number">0x00</span>, <span class="number">0x00</span>, <span class="number">0x00</span>, <span class="number">0x50</span>, <span class="number">0x00</span>, <span class="number">0x00</span>, <span class="number">0x38</span>, <span class="number">0x00</span>, <span class="number">0x00</span>,
        <span class="number">0x00</span>, <span class="number">0x10</span>, <span class="number">0x10</span>, <span class="number">0x00</span>, <span class="number">0x00</span>, <span class="number">0x38</span>, <span class="number">0x00</span>, <span class="number">0x00</span>, <span class="number">0x00</span>, <span class="number">0x00</span>, <span class="number">0x00</span>, <span class="number">0x00</span>, <span class="number">0x00</span>, <span class="number">0x00</span>, <span class="number">0x00</span>,
        <span class="number">0x00</span>, <span class="number">0x00</span>, <span class="number">0x00</span>, <span class="number">0x00</span>, <span class="number">0x00</span>, <span class="number">0x00</span>, <span class="number">0x00</span>, <span class="number">0x00</span>, <span class="number">0x00</span>, <span class="number">0x00</span>, <span class="number">0x00</span>, <span class="number">0x00</span>, <span class="number">0x00</span>, <span class="number">0x00</span>, <span class="number">0x00</span>,
        <span class="number">0x00</span>, <span class="number">0x00</span>, <span class="number">0x00</span>, <span class="number">0x68</span>, <span class="number">0x10</span>, <span class="number">0x00</span>, <span class="number">0x00</span>, <span class="number">0x5c</span>, <span class="number">0x00</span>, <span class="number">0x00</span>, <span class="number">0x00</span>, <span class="number">0x00</span>, <span class="number">0x00</span>, <span class="number">0x00</span>, <span class="number">0x00</span>,
        <span class="number">0x00</span>, <span class="number">0x00</span>, <span class="number">0x00</span>, <span class="number">0x00</span>, <span class="number">0x00</span>, <span class="number">0x30</span>, <span class="number">0x00</span>, <span class="number">0x00</span>, <span class="number">0x3c</span>, <span class="number">0x00</span>, <span class="number">0x00</span>, <span class="number">0x00</span>, <span class="number">0x00</span>, <span class="number">0x00</span>, <span class="number">0x00</span>,
        <span class="number">0x00</span>, <span class="number">0x00</span>, <span class="number">0x00</span>, <span class="number">0x00</span>, <span class="number">0x00</span>, <span class="number">0x00</span>, <span class="number">0x00</span>, <span class="number">0x00</span>, <span class="number">0x00</span>, <span class="number">0x00</span>, <span class="number">0x00</span>, <span class="number">0x00</span>, <span class="number">0x00</span>, <span class="number">0x00</span>, <span class="number">0x00</span>,
        <span class="number">0x00</span>, <span class="number">0x00</span>, <span class="number">0x00</span>, <span class="number">0x00</span>, <span class="number">0x00</span>, <span class="number">0x00</span>, <span class="number">0x2e</span>, <span class="number">0x74</span>, <span class="number">0x65</span>, <span class="number">0x78</span>, <span class="number">0x74</span>, <span class="number">0x00</span>, <span class="number">0x00</span>, <span class="number">0x00</span>, <span class="number">0x24</span>,
        <span class="number">0x06</span>, <span class="number">0x00</span>, <span class="number">0x00</span>, <span class="number">0x00</span>, <span class="number">0x10</span>, <span class="number">0x00</span>, <span class="number">0x00</span>, <span class="number">0x00</span>, <span class="number">0x08</span>, <span class="number">0x00</span>, <span class="number">0x00</span>, <span class="number">0x00</span>, <span class="number">0x04</span>, <span class="number">0x00</span>, <span class="number">0x00</span>,
        <span class="number">0x00</span>, <span class="number">0x00</span>, <span class="number">0x00</span>, <span class="number">0x00</span>, <span class="number">0x00</span>, <span class="number">0x00</span>, <span class="number">0x00</span>, <span class="number">0x00</span>, <span class="number">0x00</span>, <span class="number">0x00</span>, <span class="number">0x00</span>, <span class="number">0x00</span>, <span class="number">0x20</span>, <span class="number">0x00</span>, <span class="number">0x00</span>,
        <span class="number">0x60</span>, <span class="number">0x2e</span>, <span class="number">0x64</span>, <span class="number">0x61</span>, <span class="number">0x74</span>, <span class="number">0x61</span>, <span class="number">0x00</span>, <span class="number">0x00</span>, <span class="number">0x00</span>, <span class="number">0x3c</span>, <span class="number">0x03</span>, <span class="number">0x00</span>, <span class="number">0x00</span>, <span class="number">0x00</span>, <span class="number">0x20</span>,
        <span class="number">0x00</span>, <span class="number">0x00</span>, <span class="number">0x00</span>, <span class="number">0x02</span>, <span class="number">0x00</span>, <span class="number">0x00</span>, <span class="number">0x00</span>, <span class="number">0x0c</span>, <span class="number">0x00</span>, <span class="number">0x00</span>, <span class="number">0x00</span>, <span class="number">0x00</span>, <span class="number">0x00</span>, <span class="number">0x00</span>, <span class="number">0x00</span>,
        <span class="number">0x00</span>, <span class="number">0x00</span>, <span class="number">0x00</span>, <span class="number">0x00</span>, <span class="number">0x00</span>, <span class="number">0x00</span>, <span class="number">0x00</span>, <span class="number">0x40</span>, <span class="number">0x00</span>, <span class="number">0x00</span>, <span class="number">0xc0</span>, <span class="number">0x2e</span>, <span class="number">0x69</span>, <span class="number">0x64</span>, <span class="number">0x61</span>,
        <span class="number">0x74</span>, <span class="number">0x61</span>, <span class="number">0x00</span>, <span class="number">0x00</span>, <span class="number">0xf8</span>, <span class="number">0x01</span>, <span class="number">0x00</span>, <span class="number">0x00</span>, <span class="number">0x00</span>, <span class="number">0x30</span>, <span class="number">0x00</span>, <span class="number">0x00</span>, <span class="number">0x00</span>, <span class="number">0x02</span>, <span class="number">0x00</span>,
        <span class="number">0x00</span>, <span class="number">0x00</span>, <span class="number">0x0e</span>, <span class="number">0x00</span>, <span class="number">0x00</span>, <span class="number">0x00</span>, <span class="number">0x00</span>, <span class="number">0x00</span>, <span class="number">0x00</span>, <span class="number">0x00</span>, <span class="number">0x00</span>, <span class="number">0x00</span>, <span class="number">0x00</span>, <span class="number">0x00</span>, <span class="number">0x00</span>,
        <span class="number">0x00</span>, <span class="number">0x00</span>, <span class="number">0x40</span>, <span class="number">0x00</span>, <span class="number">0x00</span>, <span class="number">0x40</span>, <span class="number">0x2e</span>, <span class="number">0x72</span>, <span class="number">0x73</span>, <span class="number">0x72</span>, <span class="number">0x63</span>, <span class="number">0x00</span>, <span class="number">0x00</span>, <span class="number">0x00</span>, <span class="number">0x00</span>,
        <span class="number">0x08</span>, <span class="number">0x00</span>, <span class="number">0x00</span>, <span class="number">0x00</span>, <span class="number">0x40</span>, <span class="number">0x00</span>, <span class="number">0x00</span>, <span class="number">0x00</span>, <span class="number">0x08</span>, <span class="number">0x00</span>, <span class="number">0x00</span>, <span class="number">0x00</span>, <span class="number">0x10</span>, <span class="number">0x00</span>, <span class="number">0x00</span>,
        <span class="number">0x00</span>, <span class="number">0x00</span>, <span class="number">0x00</span>, <span class="number">0x00</span>, <span class="number">0x00</span>, <span class="number">0x00</span>, <span class="number">0x00</span>, <span class="number">0x00</span>, <span class="number">0x00</span>, <span class="number">0x00</span>, <span class="number">0x00</span>, <span class="number">0x00</span>, <span class="number">0x40</span>, <span class="number">0x00</span>, <span class="number">0x00</span>,
        <span class="number">0x42</span>, <span class="number">0x2e</span>, <span class="number">0x72</span>, <span class="number">0x65</span>, <span class="number">0x6c</span>, <span class="number">0x6f</span>, <span class="number">0x63</span>, <span class="number">0x00</span>, <span class="number">0x00</span>, <span class="number">0x86</span>, <span class="number">0x01</span>, <span class="number">0x00</span>, <span class="number">0x00</span>, <span class="number">0x00</span>, <span class="number">0x50</span>,
        <span class="number">0x00</span>, <span class="number">0x00</span>, <span class="number">0x00</span>, <span class="number">0x02</span>, <span class="number">0x00</span>, <span class="number">0x00</span>, <span class="number">0x00</span>, <span class="number">0x18</span>, <span class="number">0x00</span>, <span class="number">0x00</span>, <span class="number">0x00</span>, <span class="number">0x00</span>, <span class="number">0x00</span>, <span class="number">0x00</span>, <span class="number">0x00</span>,
        <span class="number">0x00</span>, <span class="number">0x00</span>, <span class="number">0x00</span>, <span class="number">0x00</span>, <span class="number">0x00</span>, <span class="number">0x00</span>, <span class="number">0x00</span>, <span class="number">0x40</span>, <span class="number">0x00</span>, <span class="number">0x00</span>, <span class="number">0x42</span>, <span class="number">0x00</span>, <span class="number">0x00</span>, <span class="number">0x00</span>, <span class="number">0x00</span>,
        <span class="number">0x00</span>, <span class="number">0x00</span>, <span class="number">0x00</span>, <span class="number">0x00</span>, <span class="number">0x00</span>, <span class="number">0x00</span>, <span class="number">0x00</span>, <span class="number">0x00</span>, <span class="number">0x00</span>, <span class="number">0x00</span>, <span class="number">0x00</span>, <span class="number">0x00</span>, <span class="number">0x00</span>, <span class="number">0x00</span>, <span class="number">0x00</span>,
        <span class="number">0x00</span>, <span class="number">0x00</span>, <span class="number">0x00</span>, <span class="number">0x00</span>, <span class="number">0x00</span>, <span class="number">0x00</span>, <span class="number">0x00</span>, <span class="number">0x00</span>, <span class="number">0x00</span>, <span class="number">0x00</span>, <span class="number">0x00</span>, <span class="number">0x00</span>, <span class="number">0x00</span>,
    ];

    <span class="attr">#[test]
    </span><span class="kw">fn </span>crss_header() {
        <span class="kw">let </span>header = Header::parse(&amp;&amp;CRSS_HEADER[..]).unwrap();
        <span class="macro">assert!</span>(header.dos_header.signature == DOS_MAGIC);
        <span class="macro">assert!</span>(header.signature == PE_MAGIC);
        <span class="macro">assert!</span>(header.coff_header.machine == COFF_MACHINE_X86);
        <span class="macro">println!</span>(<span class="string">"header: {:?}"</span>, <span class="kw-2">&amp;</span>header);
    }
}
</code></pre></div></section></main></body></html>