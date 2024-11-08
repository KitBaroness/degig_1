<!DOCTYPE html><html lang="en"><head><meta charset="utf-8"><meta name="viewport" content="width=device-width, initial-scale=1.0"><meta name="generator" content="rustdoc"><meta name="description" content="Source of the Rust file `/Users/jeyakumar/.cargo/registry/src/index.crates.io-6f17d22bba15001f/goblin-0.5.4/src/pe/relocation.rs`."><title>relocation.rs - source</title><script>if(window.location.protocol!=="file:")document.head.insertAdjacentHTML("beforeend","SourceSerif4-Regular-46f98efaafac5295.ttf.woff2,FiraSans-Regular-018c141bf0843ffd.woff2,FiraSans-Medium-8f9a781e4970d388.woff2,SourceCodePro-Regular-562dcc5011b6de7d.ttf.woff2,SourceCodePro-Semibold-d899c5a5c4aeb14a.ttf.woff2".split(",").map(f=>`<link rel="preload" as="font" type="font/woff2" crossorigin href="../../../static.files/${f}">`).join(""))</script><link rel="stylesheet" href="../../../static.files/normalize-76eba96aa4d2e634.css"><link rel="stylesheet" href="../../../static.files/rustdoc-081576b923113409.css"><meta name="rustdoc-vars" data-root-path="../../../" data-static-root-path="../../../static.files/" data-current-crate="goblin" data-themes="" data-resource-suffix="" data-rustdoc-version="1.79.0 (129f3b996 2024-06-10)" data-channel="1.79.0" data-search-js="search-bf21c90c8c1d92b1.js" data-settings-js="settings-4313503d2e1961c2.js" ><script src="../../../static.files/storage-e32f0c247825364d.js"></script><script defer src="../../../static.files/src-script-e66d777a5a92e9b2.js"></script><script defer src="../../../src-files.js"></script><script defer src="../../../static.files/main-20a3ad099b048cf2.js"></script><noscript><link rel="stylesheet" href="../../../static.files/noscript-09095024cf37855e.css"></noscript><link rel="alternate icon" type="image/png" href="../../../static.files/favicon-32x32-422f7d1d52889060.png"><link rel="icon" type="image/svg+xml" href="../../../static.files/favicon-2c020d218678b618.svg"></head><body class="rustdoc src"><!--[if lte IE 11]><div class="warning">This old browser is unsupported and will most likely display funky things.</div><![endif]--><nav class="sidebar"><div class="src-sidebar-title"><h2>Files</h2></div></nav><div class="sidebar-resizer"></div><main><nav class="sub"><form class="search-form"><span></span><div id="sidebar-button" tabindex="-1"><a href="../../../goblin/all.html" title="show sidebar"></a></div><input class="search-input" name="search" aria-label="Run search in the documentation" autocomplete="off" spellcheck="false" placeholder="Type ‘S’ or ‘/’ to search, ‘?’ for more options…" type="search"><div id="help-button" tabindex="-1"><a href="../../../help.html" title="help">?</a></div><div id="settings-menu" tabindex="-1"><a href="../../../settings.html" title="settings">Settings</a></div></form></nav><section id="main-content" class="content"><div class="example-wrap"><div data-nosnippet><pre class="src-line-numbers"><a href="#1" id="1">1</a>
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
</pre></div><pre class="rust"><code><span class="kw">use </span><span class="kw">crate</span>::error;
<span class="kw">use </span>scroll::{IOread, IOwrite, Pread, Pwrite, SizeWith};

<span class="doccomment">/// Size of a single COFF relocation.
</span><span class="kw">pub const </span>COFF_RELOCATION_SIZE: usize = <span class="number">10</span>;

<span class="comment">// x86 relocations.

</span><span class="doccomment">/// The relocation is ignored.
</span><span class="kw">pub const </span>IMAGE_REL_I386_ABSOLUTE: u16 = <span class="number">0x0000</span>;
<span class="doccomment">/// Not supported.
</span><span class="kw">pub const </span>IMAGE_REL_I386_DIR16: u16 = <span class="number">0x0001</span>;
<span class="doccomment">/// Not supported.
</span><span class="kw">pub const </span>IMAGE_REL_I386_REL16: u16 = <span class="number">0x0002</span>;
<span class="doccomment">/// The target's 32-bit VA.
</span><span class="kw">pub const </span>IMAGE_REL_I386_DIR32: u16 = <span class="number">0x0006</span>;
<span class="doccomment">/// The target's 32-bit RVA.
</span><span class="kw">pub const </span>IMAGE_REL_I386_DIR32NB: u16 = <span class="number">0x0007</span>;
<span class="doccomment">/// Not supported.
</span><span class="kw">pub const </span>IMAGE_REL_I386_SEG12: u16 = <span class="number">0x0009</span>;
<span class="doccomment">/// The 16-bit section index of the section that contains the target.
///
/// This is used to support debugging information.
</span><span class="kw">pub const </span>IMAGE_REL_I386_SECTION: u16 = <span class="number">0x000A</span>;
<span class="doccomment">/// The 32-bit offset of the target from the beginning of its section.
///
/// This is used to support debugging information and static thread local storage.
</span><span class="kw">pub const </span>IMAGE_REL_I386_SECREL: u16 = <span class="number">0x000B</span>;
<span class="doccomment">/// The CLR token.
</span><span class="kw">pub const </span>IMAGE_REL_I386_TOKEN: u16 = <span class="number">0x000C</span>;
<span class="doccomment">/// A 7-bit offset from the base of the section that contains the target.
</span><span class="kw">pub const </span>IMAGE_REL_I386_SECREL7: u16 = <span class="number">0x000D</span>;
<span class="doccomment">/// The 32-bit relative displacement to the target.
///
/// This supports the x86 relative branch and call instructions.
</span><span class="kw">pub const </span>IMAGE_REL_I386_REL32: u16 = <span class="number">0x0014</span>;

<span class="comment">// x86-64 relocations.

</span><span class="doccomment">/// The relocation is ignored.
</span><span class="kw">pub const </span>IMAGE_REL_AMD64_ABSOLUTE: u16 = <span class="number">0x0000</span>;
<span class="doccomment">/// The 64-bit VA of the relocation target.
</span><span class="kw">pub const </span>IMAGE_REL_AMD64_ADDR64: u16 = <span class="number">0x0001</span>;
<span class="doccomment">/// The 32-bit VA of the relocation target.
</span><span class="kw">pub const </span>IMAGE_REL_AMD64_ADDR32: u16 = <span class="number">0x0002</span>;
<span class="doccomment">/// The 32-bit address without an image base (RVA).
</span><span class="kw">pub const </span>IMAGE_REL_AMD64_ADDR32NB: u16 = <span class="number">0x0003</span>;
<span class="doccomment">/// The 32-bit relative address from the byte following the relocation.
</span><span class="kw">pub const </span>IMAGE_REL_AMD64_REL32: u16 = <span class="number">0x0004</span>;
<span class="doccomment">/// The 32-bit address relative to byte distance 1 from the relocation.
</span><span class="kw">pub const </span>IMAGE_REL_AMD64_REL32_1: u16 = <span class="number">0x0005</span>;
<span class="doccomment">/// The 32-bit address relative to byte distance 2 from the relocation.
</span><span class="kw">pub const </span>IMAGE_REL_AMD64_REL32_2: u16 = <span class="number">0x0006</span>;
<span class="doccomment">/// The 32-bit address relative to byte distance 3 from the relocation.
</span><span class="kw">pub const </span>IMAGE_REL_AMD64_REL32_3: u16 = <span class="number">0x0007</span>;
<span class="doccomment">/// The 32-bit address relative to byte distance 4 from the relocation.
</span><span class="kw">pub const </span>IMAGE_REL_AMD64_REL32_4: u16 = <span class="number">0x0008</span>;
<span class="doccomment">/// The 32-bit address relative to byte distance 5 from the relocation.
</span><span class="kw">pub const </span>IMAGE_REL_AMD64_REL32_5: u16 = <span class="number">0x0009</span>;
<span class="doccomment">/// The 16-bit section index of the section that contains the target.
///
/// This is used to support debugging information.
</span><span class="kw">pub const </span>IMAGE_REL_AMD64_SECTION: u16 = <span class="number">0x000A</span>;
<span class="doccomment">/// The 32-bit offset of the target from the beginning of its section.
///
/// This is used to support debugging information and static thread local storage.
</span><span class="kw">pub const </span>IMAGE_REL_AMD64_SECREL: u16 = <span class="number">0x000B</span>;
<span class="doccomment">/// A 7-bit unsigned offset from the base of the section that contains the target.
</span><span class="kw">pub const </span>IMAGE_REL_AMD64_SECREL7: u16 = <span class="number">0x000C</span>;
<span class="doccomment">/// CLR tokens.
</span><span class="kw">pub const </span>IMAGE_REL_AMD64_TOKEN: u16 = <span class="number">0x000D</span>;
<span class="doccomment">/// A 32-bit signed span-dependent value emitted into the object.
</span><span class="kw">pub const </span>IMAGE_REL_AMD64_SREL32: u16 = <span class="number">0x000E</span>;
<span class="doccomment">/// A pair that must immediately follow every span-dependent value.
</span><span class="kw">pub const </span>IMAGE_REL_AMD64_PAIR: u16 = <span class="number">0x000F</span>;
<span class="doccomment">/// A 32-bit signed span-dependent value that is applied at link time.
</span><span class="kw">pub const </span>IMAGE_REL_AMD64_SSPAN32: u16 = <span class="number">0x0010</span>;

<span class="doccomment">/// A COFF relocation.
</span><span class="attr">#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Default, Pread, Pwrite, IOread, IOwrite, SizeWith)]
</span><span class="kw">pub struct </span>Relocation {
    <span class="doccomment">/// The address of the item to which relocation is applied.
    ///
    /// This is the offset from the beginning of the section, plus the
    /// value of the section's `virtual_address` field.
    </span><span class="kw">pub </span>virtual_address: u32,
    <span class="doccomment">/// A zero-based index into the symbol table.
    ///
    /// This symbol gives the address that is to be used for the relocation. If the specified
    /// symbol has section storage class, then the symbol's address is the address with the
    /// first section of the same name.
    </span><span class="kw">pub </span>symbol_table_index: u32,
    <span class="doccomment">/// A value that indicates the kind of relocation that should be performed.
    ///
    /// Valid relocation types depend on machine type.
    </span><span class="kw">pub </span>typ: u16,
}

<span class="doccomment">/// An iterator for COFF relocations.
</span><span class="attr">#[derive(Default)]
</span><span class="kw">pub struct </span>Relocations&lt;<span class="lifetime">'a</span>&gt; {
    offset: usize,
    relocations: <span class="kw-2">&amp;</span><span class="lifetime">'a </span>[u8],
}

<span class="kw">impl</span>&lt;<span class="lifetime">'a</span>&gt; Relocations&lt;<span class="lifetime">'a</span>&gt; {
    <span class="doccomment">/// Parse a COFF relocation table at the given offset.
    ///
    /// The offset and number of relocations should be from the COFF section header.
    </span><span class="kw">pub fn </span>parse(bytes: <span class="kw-2">&amp;</span><span class="lifetime">'a </span>[u8], offset: usize, number: usize) -&gt; error::Result&lt;Relocations&lt;<span class="lifetime">'a</span>&gt;&gt; {
        <span class="kw">let </span>relocations = bytes.pread_with(offset, number * COFF_RELOCATION_SIZE)<span class="question-mark">?</span>;
        <span class="prelude-val">Ok</span>(Relocations {
            offset: <span class="number">0</span>,
            relocations,
        })
    }
}

<span class="kw">impl</span>&lt;<span class="lifetime">'a</span>&gt; Iterator <span class="kw">for </span>Relocations&lt;<span class="lifetime">'a</span>&gt; {
    <span class="kw">type </span>Item = Relocation;
    <span class="kw">fn </span>next(<span class="kw-2">&amp;mut </span><span class="self">self</span>) -&gt; <span class="prelude-ty">Option</span>&lt;<span class="self">Self</span>::Item&gt; {
        <span class="kw">if </span><span class="self">self</span>.offset &gt;= <span class="self">self</span>.relocations.len() {
            <span class="prelude-val">None
        </span>} <span class="kw">else </span>{
            <span class="prelude-val">Some</span>(
                <span class="self">self</span>.relocations
                    .gread_with(<span class="kw-2">&amp;mut </span><span class="self">self</span>.offset, scroll::LE)
                    .unwrap(),
            )
        }
    }
}
</code></pre></div></section></main></body></html>