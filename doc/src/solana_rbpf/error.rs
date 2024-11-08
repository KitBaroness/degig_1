<!DOCTYPE html><html lang="en"><head><meta charset="utf-8"><meta name="viewport" content="width=device-width, initial-scale=1.0"><meta name="generator" content="rustdoc"><meta name="description" content="Source of the Rust file `/Users/jeyakumar/.cargo/registry/src/index.crates.io-6f17d22bba15001f/solana_rbpf-0.8.0/src/error.rs`."><title>error.rs - source</title><script>if(window.location.protocol!=="file:")document.head.insertAdjacentHTML("beforeend","SourceSerif4-Regular-46f98efaafac5295.ttf.woff2,FiraSans-Regular-018c141bf0843ffd.woff2,FiraSans-Medium-8f9a781e4970d388.woff2,SourceCodePro-Regular-562dcc5011b6de7d.ttf.woff2,SourceCodePro-Semibold-d899c5a5c4aeb14a.ttf.woff2".split(",").map(f=>`<link rel="preload" as="font" type="font/woff2" crossorigin href="../../static.files/${f}">`).join(""))</script><link rel="stylesheet" href="../../static.files/normalize-76eba96aa4d2e634.css"><link rel="stylesheet" href="../../static.files/rustdoc-081576b923113409.css"><meta name="rustdoc-vars" data-root-path="../../" data-static-root-path="../../static.files/" data-current-crate="solana_rbpf" data-themes="" data-resource-suffix="" data-rustdoc-version="1.79.0 (129f3b996 2024-06-10)" data-channel="1.79.0" data-search-js="search-bf21c90c8c1d92b1.js" data-settings-js="settings-4313503d2e1961c2.js" ><script src="../../static.files/storage-e32f0c247825364d.js"></script><script defer src="../../static.files/src-script-e66d777a5a92e9b2.js"></script><script defer src="../../src-files.js"></script><script defer src="../../static.files/main-20a3ad099b048cf2.js"></script><noscript><link rel="stylesheet" href="../../static.files/noscript-09095024cf37855e.css"></noscript><link rel="icon" href="https://raw.githubusercontent.com/qmonnet/rbpf/master/misc/rbpf.ico"></head><body class="rustdoc src"><!--[if lte IE 11]><div class="warning">This old browser is unsupported and will most likely display funky things.</div><![endif]--><nav class="sidebar"><div class="src-sidebar-title"><h2>Files</h2></div></nav><div class="sidebar-resizer"></div><main><nav class="sub"><form class="search-form"><span></span><div id="sidebar-button" tabindex="-1"><a href="../../solana_rbpf/all.html" title="show sidebar"></a></div><input class="search-input" name="search" aria-label="Run search in the documentation" autocomplete="off" spellcheck="false" placeholder="Type ‘S’ or ‘/’ to search, ‘?’ for more options…" type="search"><div id="help-button" tabindex="-1"><a href="../../help.html" title="help">?</a></div><div id="settings-menu" tabindex="-1"><a href="../../settings.html" title="settings">Settings</a></div></form></nav><section id="main-content" class="content"><div class="example-wrap"><div data-nosnippet><pre class="src-line-numbers"><a href="#1" id="1">1</a>
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
</pre></div><pre class="rust"><code><span class="comment">// Copyright 2016 6WIND S.A. &lt;quentin.monnet@6wind.com&gt;
//
// Licensed under the Apache License, Version 2.0 &lt;http://www.apache.org/licenses/LICENSE-2.0&gt; or
// the MIT license &lt;http://opensource.org/licenses/MIT&gt;, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

</span><span class="doccomment">//! This module contains all the definitions related to eBPF, and some functions permitting to
//! manipulate eBPF instructions.
//!
//! The number of bytes in an instruction, the maximum number of instructions in a program, and
//! also all operation codes are defined here as constants.
//!
//! The structure for an instruction used by this crate, as well as the function to extract it from
//! a program, is also defined in the module.
//!
//! To learn more about these instructions, see the Linux kernel documentation:
//! &lt;https://www.kernel.org/doc/Documentation/networking/filter.txt&gt;, or for a shorter version of
//! the list of the operation codes: &lt;https://github.com/iovisor/bpf-docs/blob/master/eBPF.md&gt;

</span><span class="kw">use </span>{
    <span class="kw">crate</span>::{elf::ElfError, memory_region::AccessType, verifier::VerifierError},
    std::error::Error,
};

<span class="doccomment">/// Error definitions
</span><span class="attr">#[derive(Debug, thiserror::Error)]
#[repr(u64)] </span><span class="comment">// discriminant size, used in emit_exception_kind in JIT
</span><span class="kw">pub enum </span>EbpfError {
    <span class="doccomment">/// ELF error
    </span><span class="attr">#[error(<span class="string">"ELF error: {0}"</span>)]
    </span>ElfError(<span class="attr">#[from] </span>ElfError),
    <span class="doccomment">/// Function was already registered
    </span><span class="attr">#[error(<span class="string">"function #{0} was already registered"</span>)]
    </span>FunctionAlreadyRegistered(usize),
    <span class="doccomment">/// Exceeded max BPF to BPF call depth
    </span><span class="attr">#[error(<span class="string">"exceeded max BPF to BPF call depth"</span>)]
    </span>CallDepthExceeded,
    <span class="doccomment">/// Attempt to exit from root call frame
    </span><span class="attr">#[error(<span class="string">"attempted to exit root call frame"</span>)]
    </span>ExitRootCallFrame,
    <span class="doccomment">/// Divide by zero"
    </span><span class="attr">#[error(<span class="string">"divide by zero at BPF instruction"</span>)]
    </span>DivideByZero,
    <span class="doccomment">/// Divide overflow
    </span><span class="attr">#[error(<span class="string">"division overflow at BPF instruction"</span>)]
    </span>DivideOverflow,
    <span class="doccomment">/// Exceeded max instructions allowed
    </span><span class="attr">#[error(<span class="string">"attempted to execute past the end of the text segment at BPF instruction"</span>)]
    </span>ExecutionOverrun,
    <span class="doccomment">/// Attempt to call to an address outside the text segment
    </span><span class="attr">#[error(<span class="string">"callx attempted to call outside of the text segment"</span>)]
    </span>CallOutsideTextSegment,
    <span class="doccomment">/// Exceeded max instructions allowed
    </span><span class="attr">#[error(<span class="string">"exceeded CUs meter at BPF instruction"</span>)]
    </span>ExceededMaxInstructions,
    <span class="doccomment">/// Program has not been JIT-compiled
    </span><span class="attr">#[error(<span class="string">"program has not been JIT-compiled"</span>)]
    </span>JitNotCompiled,
    <span class="doccomment">/// Invalid virtual address
    </span><span class="attr">#[error(<span class="string">"invalid virtual address {0:x?}"</span>)]
    </span>InvalidVirtualAddress(u64),
    <span class="doccomment">/// Memory region index or virtual address space is invalid
    </span><span class="attr">#[error(<span class="string">"Invalid memory region at index {0}"</span>)]
    </span>InvalidMemoryRegion(usize),
    <span class="doccomment">/// Access violation (general)
    </span><span class="attr">#[error(<span class="string">"Access violation in {3} section at address {1:#x} of size {2:?}"</span>)]
    </span>AccessViolation(AccessType, u64, u64, <span class="kw-2">&amp;</span><span class="lifetime">'static </span>str),
    <span class="doccomment">/// Access violation (stack specific)
    </span><span class="attr">#[error(<span class="string">"Access violation in stack frame {3} at address {1:#x} of size {2:?}"</span>)]
    </span>StackAccessViolation(AccessType, u64, u64, i64),
    <span class="doccomment">/// Invalid instruction
    </span><span class="attr">#[error(<span class="string">"invalid BPF instruction"</span>)]
    </span>InvalidInstruction,
    <span class="doccomment">/// Unsupported instruction
    </span><span class="attr">#[error(<span class="string">"unsupported BPF instruction"</span>)]
    </span>UnsupportedInstruction,
    <span class="doccomment">/// Compilation is too big to fit
    </span><span class="attr">#[error(<span class="string">"Compilation exhausted text segment at BPF instruction {0}"</span>)]
    </span>ExhaustedTextSegment(usize),
    <span class="doccomment">/// Libc function call returned an error
    </span><span class="attr">#[error(<span class="string">"Libc calling {0} {1:?} returned error code {2}"</span>)]
    </span>LibcInvocationFailed(<span class="kw-2">&amp;</span><span class="lifetime">'static </span>str, Vec&lt;String&gt;, i32),
    <span class="doccomment">/// Verifier error
    </span><span class="attr">#[error(<span class="string">"Verifier error: {0}"</span>)]
    </span>VerifierError(<span class="attr">#[from] </span>VerifierError),
    <span class="doccomment">/// Syscall error
    </span><span class="attr">#[error(<span class="string">"Syscall error: {0}"</span>)]
    </span>SyscallError(Box&lt;<span class="kw">dyn </span>Error&gt;),
}

<span class="doccomment">/// Same as `Result` but provides a stable memory layout
</span><span class="attr">#[derive(Debug)]
#[repr(C, u64)]
</span><span class="kw">pub enum </span>StableResult&lt;T, E&gt; {
    <span class="doccomment">/// Success
    </span><span class="prelude-val">Ok</span>(T),
    <span class="doccomment">/// Failure
    </span><span class="prelude-val">Err</span>(E),
}

<span class="kw">impl</span>&lt;T: std::fmt::Debug, E: std::fmt::Debug&gt; StableResult&lt;T, E&gt; {
    <span class="doccomment">/// `true` if `Ok`
    </span><span class="kw">pub fn </span>is_ok(<span class="kw-2">&amp;</span><span class="self">self</span>) -&gt; bool {
        <span class="kw">match </span><span class="self">self </span>{
            <span class="self">Self</span>::Ok(<span class="kw">_</span>) =&gt; <span class="bool-val">true</span>,
            <span class="self">Self</span>::Err(<span class="kw">_</span>) =&gt; <span class="bool-val">false</span>,
        }
    }

    <span class="doccomment">/// `true` if `Err`
    </span><span class="kw">pub fn </span>is_err(<span class="kw-2">&amp;</span><span class="self">self</span>) -&gt; bool {
        <span class="kw">match </span><span class="self">self </span>{
            <span class="self">Self</span>::Ok(<span class="kw">_</span>) =&gt; <span class="bool-val">false</span>,
            <span class="self">Self</span>::Err(<span class="kw">_</span>) =&gt; <span class="bool-val">true</span>,
        }
    }

    <span class="doccomment">/// Returns the inner value if `Ok`, panics otherwise
    </span><span class="kw">pub fn </span>unwrap(<span class="self">self</span>) -&gt; T {
        <span class="kw">match </span><span class="self">self </span>{
            <span class="self">Self</span>::Ok(value) =&gt; value,
            <span class="self">Self</span>::Err(error) =&gt; <span class="macro">panic!</span>(<span class="string">"unwrap {:?}"</span>, error),
        }
    }

    <span class="doccomment">/// Returns the inner error if `Err`, panics otherwise
    </span><span class="kw">pub fn </span>unwrap_err(<span class="self">self</span>) -&gt; E {
        <span class="kw">match </span><span class="self">self </span>{
            <span class="self">Self</span>::Ok(value) =&gt; <span class="macro">panic!</span>(<span class="string">"unwrap_err {:?}"</span>, value),
            <span class="self">Self</span>::Err(error) =&gt; error,
        }
    }

    <span class="doccomment">/// Maps ok values, leaving error values untouched
    </span><span class="kw">pub fn </span>map&lt;U, O: FnOnce(T) -&gt; U&gt;(<span class="self">self</span>, op: O) -&gt; StableResult&lt;U, E&gt; {
        <span class="kw">match </span><span class="self">self </span>{
            <span class="self">Self</span>::Ok(value) =&gt; StableResult::&lt;U, E&gt;::Ok(op(value)),
            <span class="self">Self</span>::Err(error) =&gt; StableResult::&lt;U, E&gt;::Err(error),
        }
    }

    <span class="doccomment">/// Maps error values, leaving ok values untouched
    </span><span class="kw">pub fn </span>map_err&lt;F, O: FnOnce(E) -&gt; F&gt;(<span class="self">self</span>, op: O) -&gt; StableResult&lt;T, F&gt; {
        <span class="kw">match </span><span class="self">self </span>{
            <span class="self">Self</span>::Ok(value) =&gt; StableResult::&lt;T, F&gt;::Ok(value),
            <span class="self">Self</span>::Err(error) =&gt; StableResult::&lt;T, F&gt;::Err(op(error)),
        }
    }

    <span class="attr">#[cfg_attr(
        any(
            not(feature = <span class="string">"jit"</span>),
            target_os = <span class="string">"windows"</span>,
            not(target_arch = <span class="string">"x86_64"</span>)
        ),
        allow(dead_code)
    )]
    </span><span class="kw">pub</span>(<span class="kw">crate</span>) <span class="kw">fn </span>discriminant(<span class="kw-2">&amp;</span><span class="self">self</span>) -&gt; u64 {
        <span class="kw">unsafe </span>{ <span class="kw-2">*</span>(<span class="self">self </span><span class="kw">as </span><span class="kw-2">*const </span><span class="kw">_ as </span><span class="kw-2">*const </span>u64) }
    }
}

<span class="kw">impl</span>&lt;T, E&gt; From&lt;StableResult&lt;T, E&gt;&gt; <span class="kw">for </span><span class="prelude-ty">Result</span>&lt;T, E&gt; {
    <span class="kw">fn </span>from(result: StableResult&lt;T, E&gt;) -&gt; <span class="self">Self </span>{
        <span class="kw">match </span>result {
            StableResult::Ok(value) =&gt; <span class="prelude-val">Ok</span>(value),
            StableResult::Err(value) =&gt; <span class="prelude-val">Err</span>(value),
        }
    }
}

<span class="kw">impl</span>&lt;T, E&gt; From&lt;<span class="prelude-ty">Result</span>&lt;T, E&gt;&gt; <span class="kw">for </span>StableResult&lt;T, E&gt; {
    <span class="kw">fn </span>from(result: <span class="prelude-ty">Result</span>&lt;T, E&gt;) -&gt; <span class="self">Self </span>{
        <span class="kw">match </span>result {
            <span class="prelude-val">Ok</span>(value) =&gt; <span class="self">Self</span>::Ok(value),
            <span class="prelude-val">Err</span>(value) =&gt; <span class="self">Self</span>::Err(value),
        }
    }
}

<span class="doccomment">/// Return value of programs and syscalls
</span><span class="kw">pub type </span>ProgramResult = StableResult&lt;u64, EbpfError&gt;;

<span class="attr">#[cfg(test)]
</span><span class="kw">mod </span>tests {
    <span class="kw">use super</span>::<span class="kw-2">*</span>;

    <span class="attr">#[test]
    </span><span class="kw">fn </span>test_program_result_is_stable() {
        <span class="kw">let </span>ok = ProgramResult::Ok(<span class="number">42</span>);
        <span class="macro">assert_eq!</span>(ok.discriminant(), <span class="number">0</span>);
        <span class="kw">let </span>err = ProgramResult::Err(EbpfError::JitNotCompiled);
        <span class="macro">assert_eq!</span>(err.discriminant(), <span class="number">1</span>);
    }
}
</code></pre></div></section></main></body></html>