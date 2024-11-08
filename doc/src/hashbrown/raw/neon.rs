<!DOCTYPE html><html lang="en"><head><meta charset="utf-8"><meta name="viewport" content="width=device-width, initial-scale=1.0"><meta name="generator" content="rustdoc"><meta name="description" content="Source of the Rust file `/Users/jeyakumar/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.3/src/raw/neon.rs`."><title>neon.rs - source</title><script>if(window.location.protocol!=="file:")document.head.insertAdjacentHTML("beforeend","SourceSerif4-Regular-46f98efaafac5295.ttf.woff2,FiraSans-Regular-018c141bf0843ffd.woff2,FiraSans-Medium-8f9a781e4970d388.woff2,SourceCodePro-Regular-562dcc5011b6de7d.ttf.woff2,SourceCodePro-Semibold-d899c5a5c4aeb14a.ttf.woff2".split(",").map(f=>`<link rel="preload" as="font" type="font/woff2" crossorigin href="../../../static.files/${f}">`).join(""))</script><link rel="stylesheet" href="../../../static.files/normalize-76eba96aa4d2e634.css"><link rel="stylesheet" href="../../../static.files/rustdoc-081576b923113409.css"><meta name="rustdoc-vars" data-root-path="../../../" data-static-root-path="../../../static.files/" data-current-crate="hashbrown" data-themes="" data-resource-suffix="" data-rustdoc-version="1.79.0 (129f3b996 2024-06-10)" data-channel="1.79.0" data-search-js="search-bf21c90c8c1d92b1.js" data-settings-js="settings-4313503d2e1961c2.js" ><script src="../../../static.files/storage-e32f0c247825364d.js"></script><script defer src="../../../static.files/src-script-e66d777a5a92e9b2.js"></script><script defer src="../../../src-files.js"></script><script defer src="../../../static.files/main-20a3ad099b048cf2.js"></script><noscript><link rel="stylesheet" href="../../../static.files/noscript-09095024cf37855e.css"></noscript><link rel="alternate icon" type="image/png" href="../../../static.files/favicon-32x32-422f7d1d52889060.png"><link rel="icon" type="image/svg+xml" href="../../../static.files/favicon-2c020d218678b618.svg"></head><body class="rustdoc src"><!--[if lte IE 11]><div class="warning">This old browser is unsupported and will most likely display funky things.</div><![endif]--><nav class="sidebar"><div class="src-sidebar-title"><h2>Files</h2></div></nav><div class="sidebar-resizer"></div><main><nav class="sub"><form class="search-form"><span></span><div id="sidebar-button" tabindex="-1"><a href="../../../hashbrown/all.html" title="show sidebar"></a></div><input class="search-input" name="search" aria-label="Run search in the documentation" autocomplete="off" spellcheck="false" placeholder="Type ‘S’ or ‘/’ to search, ‘?’ for more options…" type="search"><div id="help-button" tabindex="-1"><a href="../../../help.html" title="help">?</a></div><div id="settings-menu" tabindex="-1"><a href="../../../settings.html" title="settings">Settings</a></div></form></nav><section id="main-content" class="content"><div class="example-wrap"><div data-nosnippet><pre class="src-line-numbers"><a href="#1" id="1">1</a>
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
</pre></div><pre class="rust"><code><span class="kw">use </span><span class="kw">super</span>::bitmask::BitMask;
<span class="kw">use </span><span class="kw">super</span>::EMPTY;
<span class="kw">use </span>core::arch::aarch64 <span class="kw">as </span>neon;
<span class="kw">use </span>core::mem;
<span class="kw">use </span>core::num::NonZeroU64;

<span class="kw">pub</span>(<span class="kw">crate</span>) <span class="kw">type </span>BitMaskWord = u64;
<span class="kw">pub</span>(<span class="kw">crate</span>) <span class="kw">type </span>NonZeroBitMaskWord = NonZeroU64;
<span class="kw">pub</span>(<span class="kw">crate</span>) <span class="kw">const </span>BITMASK_STRIDE: usize = <span class="number">8</span>;
<span class="kw">pub</span>(<span class="kw">crate</span>) <span class="kw">const </span>BITMASK_MASK: BitMaskWord = !<span class="number">0</span>;
<span class="kw">pub</span>(<span class="kw">crate</span>) <span class="kw">const </span>BITMASK_ITER_MASK: BitMaskWord = <span class="number">0x8080_8080_8080_8080</span>;

<span class="doccomment">/// Abstraction over a group of control bytes which can be scanned in
/// parallel.
///
/// This implementation uses a 64-bit NEON value.
</span><span class="attr">#[derive(Copy, Clone)]
</span><span class="kw">pub</span>(<span class="kw">crate</span>) <span class="kw">struct </span>Group(neon::uint8x8_t);

<span class="attr">#[allow(clippy::use_self)]
</span><span class="kw">impl </span>Group {
    <span class="doccomment">/// Number of bytes in the group.
    </span><span class="kw">pub</span>(<span class="kw">crate</span>) <span class="kw">const </span>WIDTH: usize = mem::size_of::&lt;<span class="self">Self</span>&gt;();

    <span class="doccomment">/// Returns a full group of empty bytes, suitable for use as the initial
    /// value for an empty hash table.
    ///
    /// This is guaranteed to be aligned to the group size.
    </span><span class="attr">#[inline]
    </span><span class="kw">pub</span>(<span class="kw">crate</span>) <span class="kw">const fn </span>static_empty() -&gt; <span class="kw-2">&amp;</span><span class="lifetime">'static </span>[u8; Group::WIDTH] {
        <span class="attr">#[repr(C)]
        </span><span class="kw">struct </span>AlignedBytes {
            _align: [Group; <span class="number">0</span>],
            bytes: [u8; Group::WIDTH],
        }
        <span class="kw">const </span>ALIGNED_BYTES: AlignedBytes = AlignedBytes {
            _align: [],
            bytes: [EMPTY; Group::WIDTH],
        };
        <span class="kw-2">&amp;</span>ALIGNED_BYTES.bytes
    }

    <span class="doccomment">/// Loads a group of bytes starting at the given address.
    </span><span class="attr">#[inline]
    #[allow(clippy::cast_ptr_alignment)] </span><span class="comment">// unaligned load
    </span><span class="kw">pub</span>(<span class="kw">crate</span>) <span class="kw">unsafe fn </span>load(ptr: <span class="kw-2">*const </span>u8) -&gt; <span class="self">Self </span>{
        Group(neon::vld1_u8(ptr))
    }

    <span class="doccomment">/// Loads a group of bytes starting at the given address, which must be
    /// aligned to `mem::align_of::&lt;Group&gt;()`.
    </span><span class="attr">#[inline]
    #[allow(clippy::cast_ptr_alignment)]
    </span><span class="kw">pub</span>(<span class="kw">crate</span>) <span class="kw">unsafe fn </span>load_aligned(ptr: <span class="kw-2">*const </span>u8) -&gt; <span class="self">Self </span>{
        <span class="comment">// FIXME: use align_offset once it stabilizes
        </span><span class="macro">debug_assert_eq!</span>(ptr <span class="kw">as </span>usize &amp; (mem::align_of::&lt;<span class="self">Self</span>&gt;() - <span class="number">1</span>), <span class="number">0</span>);
        Group(neon::vld1_u8(ptr))
    }

    <span class="doccomment">/// Stores the group of bytes to the given address, which must be
    /// aligned to `mem::align_of::&lt;Group&gt;()`.
    </span><span class="attr">#[inline]
    #[allow(clippy::cast_ptr_alignment)]
    </span><span class="kw">pub</span>(<span class="kw">crate</span>) <span class="kw">unsafe fn </span>store_aligned(<span class="self">self</span>, ptr: <span class="kw-2">*mut </span>u8) {
        <span class="comment">// FIXME: use align_offset once it stabilizes
        </span><span class="macro">debug_assert_eq!</span>(ptr <span class="kw">as </span>usize &amp; (mem::align_of::&lt;<span class="self">Self</span>&gt;() - <span class="number">1</span>), <span class="number">0</span>);
        neon::vst1_u8(ptr, <span class="self">self</span>.<span class="number">0</span>);
    }

    <span class="doccomment">/// Returns a `BitMask` indicating all bytes in the group which *may*
    /// have the given value.
    </span><span class="attr">#[inline]
    </span><span class="kw">pub</span>(<span class="kw">crate</span>) <span class="kw">fn </span>match_byte(<span class="self">self</span>, byte: u8) -&gt; BitMask {
        <span class="kw">unsafe </span>{
            <span class="kw">let </span>cmp = neon::vceq_u8(<span class="self">self</span>.<span class="number">0</span>, neon::vdup_n_u8(byte));
            BitMask(neon::vget_lane_u64(neon::vreinterpret_u64_u8(cmp), <span class="number">0</span>))
        }
    }

    <span class="doccomment">/// Returns a `BitMask` indicating all bytes in the group which are
    /// `EMPTY`.
    </span><span class="attr">#[inline]
    </span><span class="kw">pub</span>(<span class="kw">crate</span>) <span class="kw">fn </span>match_empty(<span class="self">self</span>) -&gt; BitMask {
        <span class="self">self</span>.match_byte(EMPTY)
    }

    <span class="doccomment">/// Returns a `BitMask` indicating all bytes in the group which are
    /// `EMPTY` or `DELETED`.
    </span><span class="attr">#[inline]
    </span><span class="kw">pub</span>(<span class="kw">crate</span>) <span class="kw">fn </span>match_empty_or_deleted(<span class="self">self</span>) -&gt; BitMask {
        <span class="kw">unsafe </span>{
            <span class="kw">let </span>cmp = neon::vcltz_s8(neon::vreinterpret_s8_u8(<span class="self">self</span>.<span class="number">0</span>));
            BitMask(neon::vget_lane_u64(neon::vreinterpret_u64_u8(cmp), <span class="number">0</span>))
        }
    }

    <span class="doccomment">/// Returns a `BitMask` indicating all bytes in the group which are full.
    </span><span class="attr">#[inline]
    </span><span class="kw">pub</span>(<span class="kw">crate</span>) <span class="kw">fn </span>match_full(<span class="self">self</span>) -&gt; BitMask {
        <span class="kw">unsafe </span>{
            <span class="kw">let </span>cmp = neon::vcgez_s8(neon::vreinterpret_s8_u8(<span class="self">self</span>.<span class="number">0</span>));
            BitMask(neon::vget_lane_u64(neon::vreinterpret_u64_u8(cmp), <span class="number">0</span>))
        }
    }

    <span class="doccomment">/// Performs the following transformation on all bytes in the group:
    /// - `EMPTY =&gt; EMPTY`
    /// - `DELETED =&gt; EMPTY`
    /// - `FULL =&gt; DELETED`
    </span><span class="attr">#[inline]
    </span><span class="kw">pub</span>(<span class="kw">crate</span>) <span class="kw">fn </span>convert_special_to_empty_and_full_to_deleted(<span class="self">self</span>) -&gt; <span class="self">Self </span>{
        <span class="comment">// Map high_bit = 1 (EMPTY or DELETED) to 1111_1111
        // and high_bit = 0 (FULL) to 1000_0000
        //
        // Here's this logic expanded to concrete values:
        //   let special = 0 &gt; byte = 1111_1111 (true) or 0000_0000 (false)
        //   1111_1111 | 1000_0000 = 1111_1111
        //   0000_0000 | 1000_0000 = 1000_0000
        </span><span class="kw">unsafe </span>{
            <span class="kw">let </span>special = neon::vcltz_s8(neon::vreinterpret_s8_u8(<span class="self">self</span>.<span class="number">0</span>));
            Group(neon::vorr_u8(special, neon::vdup_n_u8(<span class="number">0x80</span>)))
        }
    }
}
</code></pre></div></section></main></body></html>