<!DOCTYPE html><html lang="en"><head><meta charset="utf-8"><meta name="viewport" content="width=device-width, initial-scale=1.0"><meta name="generator" content="rustdoc"><meta name="description" content="Source of the Rust file `/Users/jeyakumar/.cargo/registry/src/index.crates.io-6f17d22bba15001f/solana_rbpf-0.8.0/src/memory_management.rs`."><title>memory_management.rs - source</title><script>if(window.location.protocol!=="file:")document.head.insertAdjacentHTML("beforeend","SourceSerif4-Regular-46f98efaafac5295.ttf.woff2,FiraSans-Regular-018c141bf0843ffd.woff2,FiraSans-Medium-8f9a781e4970d388.woff2,SourceCodePro-Regular-562dcc5011b6de7d.ttf.woff2,SourceCodePro-Semibold-d899c5a5c4aeb14a.ttf.woff2".split(",").map(f=>`<link rel="preload" as="font" type="font/woff2" crossorigin href="../../static.files/${f}">`).join(""))</script><link rel="stylesheet" href="../../static.files/normalize-76eba96aa4d2e634.css"><link rel="stylesheet" href="../../static.files/rustdoc-081576b923113409.css"><meta name="rustdoc-vars" data-root-path="../../" data-static-root-path="../../static.files/" data-current-crate="solana_rbpf" data-themes="" data-resource-suffix="" data-rustdoc-version="1.79.0 (129f3b996 2024-06-10)" data-channel="1.79.0" data-search-js="search-bf21c90c8c1d92b1.js" data-settings-js="settings-4313503d2e1961c2.js" ><script src="../../static.files/storage-e32f0c247825364d.js"></script><script defer src="../../static.files/src-script-e66d777a5a92e9b2.js"></script><script defer src="../../src-files.js"></script><script defer src="../../static.files/main-20a3ad099b048cf2.js"></script><noscript><link rel="stylesheet" href="../../static.files/noscript-09095024cf37855e.css"></noscript><link rel="icon" href="https://raw.githubusercontent.com/qmonnet/rbpf/master/misc/rbpf.ico"></head><body class="rustdoc src"><!--[if lte IE 11]><div class="warning">This old browser is unsupported and will most likely display funky things.</div><![endif]--><nav class="sidebar"><div class="src-sidebar-title"><h2>Files</h2></div></nav><div class="sidebar-resizer"></div><main><nav class="sub"><form class="search-form"><span></span><div id="sidebar-button" tabindex="-1"><a href="../../solana_rbpf/all.html" title="show sidebar"></a></div><input class="search-input" name="search" aria-label="Run search in the documentation" autocomplete="off" spellcheck="false" placeholder="Type ‘S’ or ‘/’ to search, ‘?’ for more options…" type="search"><div id="help-button" tabindex="-1"><a href="../../help.html" title="help">?</a></div><div id="settings-menu" tabindex="-1"><a href="../../settings.html" title="settings">Settings</a></div></form></nav><section id="main-content" class="content"><div class="example-wrap"><div data-nosnippet><pre class="src-line-numbers"><a href="#1" id="1">1</a>
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
</pre></div><pre class="rust"><code><span class="comment">// Copyright 2022 Solana Maintainers &lt;maintainers@solana.com&gt;
//
// Licensed under the Apache License, Version 2.0 &lt;http://www.apache.org/licenses/LICENSE-2.0&gt; or
// the MIT license &lt;http://opensource.org/licenses/MIT&gt;, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

</span><span class="attr">#![cfg_attr(target_os = <span class="string">"windows"</span>, allow(dead_code))]

</span><span class="kw">use </span><span class="kw">crate</span>::error::EbpfError;

<span class="attr">#[cfg(not(target_os = <span class="string">"windows"</span>))]
</span><span class="kw">extern crate </span>libc;
<span class="attr">#[cfg(not(target_os = <span class="string">"windows"</span>))]
</span><span class="kw">use </span>libc::c_void;

<span class="attr">#[cfg(target_os = <span class="string">"windows"</span>)]
</span><span class="kw">use </span>winapi::{
    ctypes::c_void,
    shared::minwindef,
    um::{
        errhandlingapi::GetLastError,
        memoryapi::{VirtualAlloc, VirtualFree, VirtualProtect},
        sysinfoapi::{GetSystemInfo, SYSTEM_INFO},
        winnt,
    },
};

<span class="attr">#[cfg(not(target_os = <span class="string">"windows"</span>))]
</span><span class="macro">macro_rules!</span> libc_error_guard {
    (succeeded<span class="question-mark">?</span>, mmap, <span class="macro-nonterminal">$addr</span>:expr, $(<span class="macro-nonterminal">$arg</span>:expr),<span class="kw-2">*</span>) =&gt; {{
        <span class="kw-2">*</span><span class="macro-nonterminal">$addr </span>= libc::mmap(<span class="kw-2">*</span><span class="macro-nonterminal">$addr</span>, $(<span class="macro-nonterminal">$arg</span>),<span class="kw-2">*</span>);
        <span class="kw-2">*</span><span class="macro-nonterminal">$addr </span>!= libc::MAP_FAILED
    }};
    (succeeded<span class="question-mark">?</span>, <span class="macro-nonterminal">$function</span>:ident, $(<span class="macro-nonterminal">$arg</span>:expr),<span class="kw-2">*</span>) =&gt; {
        libc::<span class="macro-nonterminal">$function</span>($(<span class="macro-nonterminal">$arg</span>),<span class="kw-2">*</span>) == <span class="number">0
    </span>};
    (<span class="macro-nonterminal">$function</span>:ident, $(<span class="macro-nonterminal">$arg</span>:expr),* $(,)<span class="question-mark">?</span>) =&gt; {{
        <span class="kw">const </span>RETRY_COUNT: usize = <span class="number">3</span>;
        <span class="kw">for </span>i <span class="kw">in </span><span class="number">0</span>..RETRY_COUNT {
            <span class="kw">if </span><span class="macro">libc_error_guard!</span>(succeeded<span class="question-mark">?</span>, <span class="macro-nonterminal">$function</span>, $(<span class="macro-nonterminal">$arg</span>),<span class="kw-2">*</span>) {
                <span class="kw">break</span>;
            } <span class="kw">else if </span>i.saturating_add(<span class="number">1</span>) == RETRY_COUNT {
                <span class="kw">let </span>args = <span class="macro">vec!</span>[$(<span class="macro">format!</span>(<span class="string">"{:?}"</span>, <span class="macro-nonterminal">$arg</span>)),<span class="kw-2">*</span>];
                <span class="attr">#[cfg(any(target_os = <span class="string">"freebsd"</span>, target_os = <span class="string">"ios"</span>, target_os = <span class="string">"macos"</span>))]
                </span><span class="kw">let </span>errno = <span class="kw-2">*</span>libc::__error();
                <span class="attr">#[cfg(any(target_os = <span class="string">"android"</span>, target_os = <span class="string">"netbsd"</span>, target_os = <span class="string">"openbsd"</span>))]
                </span><span class="kw">let </span>errno = <span class="kw-2">*</span>libc::__errno();
                <span class="attr">#[cfg(target_os = <span class="string">"linux"</span>)]
                </span><span class="kw">let </span>errno = <span class="kw-2">*</span>libc::__errno_location();
                <span class="kw">return </span><span class="prelude-val">Err</span>(EbpfError::LibcInvocationFailed(<span class="macro">stringify!</span>(<span class="macro-nonterminal">$function</span>), args, errno));
            }
        }
    }};
}

<span class="attr">#[cfg(target_os = <span class="string">"windows"</span>)]
</span><span class="macro">macro_rules!</span> winapi_error_guard {
    (succeeded<span class="question-mark">?</span>, VirtualAlloc, <span class="macro-nonterminal">$addr</span>:expr, $(<span class="macro-nonterminal">$arg</span>:expr),<span class="kw-2">*</span>) =&gt; {{
        <span class="kw-2">*</span><span class="macro-nonterminal">$addr </span>= VirtualAlloc(<span class="kw-2">*</span><span class="macro-nonterminal">$addr</span>, $(<span class="macro-nonterminal">$arg</span>),<span class="kw-2">*</span>);
        !(<span class="kw-2">*</span><span class="macro-nonterminal">$addr</span>).is_null()
    }};
    (succeeded<span class="question-mark">?</span>, <span class="macro-nonterminal">$function</span>:ident, $(<span class="macro-nonterminal">$arg</span>:expr),<span class="kw-2">*</span>) =&gt; {
        <span class="macro-nonterminal">$function</span>($(<span class="macro-nonterminal">$arg</span>),<span class="kw-2">*</span>) != <span class="number">0
    </span>};
    (<span class="macro-nonterminal">$function</span>:ident, $(<span class="macro-nonterminal">$arg</span>:expr),* $(,)<span class="question-mark">?</span>) =&gt; {{
        <span class="kw">if </span>!<span class="macro">winapi_error_guard!</span>(succeeded<span class="question-mark">?</span>, <span class="macro-nonterminal">$function</span>, $(<span class="macro-nonterminal">$arg</span>),<span class="kw-2">*</span>) {
            <span class="kw">let </span>args = <span class="macro">vec!</span>[$(<span class="macro">format!</span>(<span class="string">"{:?}"</span>, <span class="macro-nonterminal">$arg</span>)),<span class="kw-2">*</span>];
            <span class="kw">let </span>errno = GetLastError();
            <span class="kw">return </span><span class="prelude-val">Err</span>(EbpfError::LibcInvocationFailed(<span class="macro">stringify!</span>(<span class="macro-nonterminal">$function</span>), args, errno <span class="kw">as </span>i32));
        }
    }};
}

<span class="kw">pub fn </span>get_system_page_size() -&gt; usize {
    <span class="attr">#[cfg(not(target_os = <span class="string">"windows"</span>))]
    </span><span class="kw">unsafe </span>{
        libc::sysconf(libc::_SC_PAGESIZE) <span class="kw">as </span>usize
    }
    <span class="attr">#[cfg(target_os = <span class="string">"windows"</span>)]
    </span><span class="kw">unsafe </span>{
        <span class="kw">let </span><span class="kw-2">mut </span>system_info: SYSTEM_INFO = std::mem::zeroed();
        GetSystemInfo(<span class="kw-2">&amp;mut </span>system_info);
        system_info.dwPageSize <span class="kw">as </span>usize
    }
}

<span class="kw">pub fn </span>round_to_page_size(value: usize, page_size: usize) -&gt; usize {
    value
        .saturating_add(page_size)
        .saturating_sub(<span class="number">1</span>)
        .checked_div(page_size)
        .unwrap()
        .saturating_mul(page_size)
}

<span class="kw">pub unsafe fn </span>allocate_pages(size_in_bytes: usize) -&gt; <span class="prelude-ty">Result</span>&lt;<span class="kw-2">*mut </span>u8, EbpfError&gt; {
    <span class="kw">let </span><span class="kw-2">mut </span>raw: <span class="kw-2">*mut </span>c_void = std::ptr::null_mut();
    <span class="attr">#[cfg(not(target_os = <span class="string">"windows"</span>))]
    </span><span class="macro">libc_error_guard!</span>(
        mmap,
        <span class="kw-2">&amp;mut </span>raw,
        size_in_bytes,
        libc::PROT_READ | libc::PROT_WRITE,
        libc::MAP_ANONYMOUS | libc::MAP_PRIVATE,
        -<span class="number">1</span>,
        <span class="number">0</span>,
    );
    <span class="attr">#[cfg(target_os = <span class="string">"windows"</span>)]
    </span><span class="macro">winapi_error_guard!</span>(
        VirtualAlloc,
        <span class="kw-2">&amp;mut </span>raw,
        size_in_bytes,
        winnt::MEM_RESERVE | winnt::MEM_COMMIT,
        winnt::PAGE_READWRITE,
    );
    <span class="prelude-val">Ok</span>(raw <span class="kw">as </span><span class="kw-2">*mut </span>u8)
}

<span class="kw">pub unsafe fn </span>free_pages(raw: <span class="kw-2">*mut </span>u8, size_in_bytes: usize) -&gt; <span class="prelude-ty">Result</span>&lt;(), EbpfError&gt; {
    <span class="attr">#[cfg(not(target_os = <span class="string">"windows"</span>))]
    </span><span class="macro">libc_error_guard!</span>(munmap, raw <span class="kw">as </span><span class="kw-2">*mut </span><span class="kw">_</span>, size_in_bytes);
    <span class="attr">#[cfg(target_os = <span class="string">"windows"</span>)]
    </span><span class="macro">winapi_error_guard!</span>(
        VirtualFree,
        raw <span class="kw">as </span><span class="kw-2">*mut </span><span class="kw">_</span>,
        size_in_bytes,
        winnt::MEM_RELEASE, <span class="comment">// winnt::MEM_DECOMMIT
    </span>);
    <span class="prelude-val">Ok</span>(())
}

<span class="kw">pub unsafe fn </span>protect_pages(
    raw: <span class="kw-2">*mut </span>u8,
    size_in_bytes: usize,
    executable_flag: bool,
) -&gt; <span class="prelude-ty">Result</span>&lt;(), EbpfError&gt; {
    <span class="attr">#[cfg(not(target_os = <span class="string">"windows"</span>))]
    </span>{
        <span class="macro">libc_error_guard!</span>(
            mprotect,
            raw <span class="kw">as </span><span class="kw-2">*mut </span><span class="kw">_</span>,
            size_in_bytes,
            <span class="kw">if </span>executable_flag {
                libc::PROT_EXEC | libc::PROT_READ
            } <span class="kw">else </span>{
                libc::PROT_READ
            },
        );
    }
    <span class="attr">#[cfg(target_os = <span class="string">"windows"</span>)]
    </span>{
        <span class="kw">let </span><span class="kw-2">mut </span>old: minwindef::DWORD = <span class="number">0</span>;
        <span class="kw">let </span>ptr_old: <span class="kw-2">*mut </span>minwindef::DWORD = <span class="kw-2">&amp;mut </span>old;
        <span class="macro">winapi_error_guard!</span>(
            VirtualProtect,
            raw <span class="kw">as </span><span class="kw-2">*mut </span><span class="kw">_</span>,
            size_in_bytes,
            <span class="kw">if </span>executable_flag {
                winnt::PAGE_EXECUTE_READ
            } <span class="kw">else </span>{
                winnt::PAGE_READONLY
            },
            ptr_old,
        );
    }
    <span class="prelude-val">Ok</span>(())
}
</code></pre></div></section></main></body></html>