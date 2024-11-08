<!DOCTYPE html><html lang="en"><head><meta charset="utf-8"><meta name="viewport" content="width=device-width, initial-scale=1.0"><meta name="generator" content="rustdoc"><meta name="description" content="Source of the Rust file `/Users/jeyakumar/.cargo/registry/src/index.crates.io-6f17d22bba15001f/solana-program-runtime-1.18.9/src/compute_budget.rs`."><title>compute_budget.rs - source</title><script>if(window.location.protocol!=="file:")document.head.insertAdjacentHTML("beforeend","SourceSerif4-Regular-46f98efaafac5295.ttf.woff2,FiraSans-Regular-018c141bf0843ffd.woff2,FiraSans-Medium-8f9a781e4970d388.woff2,SourceCodePro-Regular-562dcc5011b6de7d.ttf.woff2,SourceCodePro-Semibold-d899c5a5c4aeb14a.ttf.woff2".split(",").map(f=>`<link rel="preload" as="font" type="font/woff2" crossorigin href="../../static.files/${f}">`).join(""))</script><link rel="stylesheet" href="../../static.files/normalize-76eba96aa4d2e634.css"><link rel="stylesheet" href="../../static.files/rustdoc-081576b923113409.css"><meta name="rustdoc-vars" data-root-path="../../" data-static-root-path="../../static.files/" data-current-crate="solana_program_runtime" data-themes="" data-resource-suffix="" data-rustdoc-version="1.79.0 (129f3b996 2024-06-10)" data-channel="1.79.0" data-search-js="search-bf21c90c8c1d92b1.js" data-settings-js="settings-4313503d2e1961c2.js" ><script src="../../static.files/storage-e32f0c247825364d.js"></script><script defer src="../../static.files/src-script-e66d777a5a92e9b2.js"></script><script defer src="../../src-files.js"></script><script defer src="../../static.files/main-20a3ad099b048cf2.js"></script><noscript><link rel="stylesheet" href="../../static.files/noscript-09095024cf37855e.css"></noscript><link rel="alternate icon" type="image/png" href="../../static.files/favicon-32x32-422f7d1d52889060.png"><link rel="icon" type="image/svg+xml" href="../../static.files/favicon-2c020d218678b618.svg"></head><body class="rustdoc src"><!--[if lte IE 11]><div class="warning">This old browser is unsupported and will most likely display funky things.</div><![endif]--><nav class="sidebar"><div class="src-sidebar-title"><h2>Files</h2></div></nav><div class="sidebar-resizer"></div><main><nav class="sub"><form class="search-form"><span></span><div id="sidebar-button" tabindex="-1"><a href="../../solana_program_runtime/all.html" title="show sidebar"></a></div><input class="search-input" name="search" aria-label="Run search in the documentation" autocomplete="off" spellcheck="false" placeholder="Type ‘S’ or ‘/’ to search, ‘?’ for more options…" type="search"><div id="help-button" tabindex="-1"><a href="../../help.html" title="help">?</a></div><div id="settings-menu" tabindex="-1"><a href="../../settings.html" title="settings">Settings</a></div></form></nav><section id="main-content" class="content"><div class="example-wrap"><div data-nosnippet><pre class="src-line-numbers"><a href="#1" id="1">1</a>
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
</pre></div><pre class="rust"><code><span class="kw">use </span>{
    <span class="kw">crate</span>::compute_budget_processor::{<span class="self">self</span>, process_compute_budget_instructions},
    solana_sdk::{instruction::CompiledInstruction, pubkey::Pubkey, transaction::Result},
};

<span class="attr">#[cfg(RUSTC_WITH_SPECIALIZATION)]
</span><span class="kw">impl </span>::solana_frozen_abi::abi_example::AbiExample <span class="kw">for </span>ComputeBudget {
    <span class="kw">fn </span>example() -&gt; <span class="self">Self </span>{
        <span class="comment">// ComputeBudget is not Serialize so just rely on Default.
        </span>ComputeBudget::default()
    }
}

<span class="doccomment">/// Roughly 0.5us/page, where page is 32K; given roughly 15CU/us, the
/// default heap page cost = 0.5 * 15 ~= 8CU/page
</span><span class="kw">pub const </span>DEFAULT_HEAP_COST: u64 = <span class="number">8</span>;

<span class="attr">#[derive(Clone, Copy, Debug, PartialEq, Eq)]
</span><span class="kw">pub struct </span>ComputeBudget {
    <span class="doccomment">/// Number of compute units that a transaction or individual instruction is
    /// allowed to consume. Compute units are consumed by program execution,
    /// resources they use, etc...
    </span><span class="kw">pub </span>compute_unit_limit: u64,
    <span class="doccomment">/// Number of compute units consumed by a log_u64 call
    </span><span class="kw">pub </span>log_64_units: u64,
    <span class="doccomment">/// Number of compute units consumed by a create_program_address call
    </span><span class="kw">pub </span>create_program_address_units: u64,
    <span class="doccomment">/// Number of compute units consumed by an invoke call (not including the cost incurred by
    /// the called program)
    </span><span class="kw">pub </span>invoke_units: u64,
    <span class="doccomment">/// Maximum program instruction invocation stack height. Invocation stack
    /// height starts at 1 for transaction instructions and the stack height is
    /// incremented each time a program invokes an instruction and decremented
    /// when a program returns.
    </span><span class="kw">pub </span>max_invoke_stack_height: usize,
    <span class="doccomment">/// Maximum cross-program invocation and instructions per transaction
    </span><span class="kw">pub </span>max_instruction_trace_length: usize,
    <span class="doccomment">/// Base number of compute units consumed to call SHA256
    </span><span class="kw">pub </span>sha256_base_cost: u64,
    <span class="doccomment">/// Incremental number of units consumed by SHA256 (based on bytes)
    </span><span class="kw">pub </span>sha256_byte_cost: u64,
    <span class="doccomment">/// Maximum number of slices hashed per syscall
    </span><span class="kw">pub </span>sha256_max_slices: u64,
    <span class="doccomment">/// Maximum SBF to BPF call depth
    </span><span class="kw">pub </span>max_call_depth: usize,
    <span class="doccomment">/// Size of a stack frame in bytes, must match the size specified in the LLVM SBF backend
    </span><span class="kw">pub </span>stack_frame_size: usize,
    <span class="doccomment">/// Number of compute units consumed by logging a `Pubkey`
    </span><span class="kw">pub </span>log_pubkey_units: u64,
    <span class="doccomment">/// Maximum cross-program invocation instruction size
    </span><span class="kw">pub </span>max_cpi_instruction_size: usize,
    <span class="doccomment">/// Number of account data bytes per compute unit charged during a cross-program invocation
    </span><span class="kw">pub </span>cpi_bytes_per_unit: u64,
    <span class="doccomment">/// Base number of compute units consumed to get a sysvar
    </span><span class="kw">pub </span>sysvar_base_cost: u64,
    <span class="doccomment">/// Number of compute units consumed to call secp256k1_recover
    </span><span class="kw">pub </span>secp256k1_recover_cost: u64,
    <span class="doccomment">/// Number of compute units consumed to do a syscall without any work
    </span><span class="kw">pub </span>syscall_base_cost: u64,
    <span class="doccomment">/// Number of compute units consumed to validate a curve25519 edwards point
    </span><span class="kw">pub </span>curve25519_edwards_validate_point_cost: u64,
    <span class="doccomment">/// Number of compute units consumed to add two curve25519 edwards points
    </span><span class="kw">pub </span>curve25519_edwards_add_cost: u64,
    <span class="doccomment">/// Number of compute units consumed to subtract two curve25519 edwards points
    </span><span class="kw">pub </span>curve25519_edwards_subtract_cost: u64,
    <span class="doccomment">/// Number of compute units consumed to multiply a curve25519 edwards point
    </span><span class="kw">pub </span>curve25519_edwards_multiply_cost: u64,
    <span class="doccomment">/// Number of compute units consumed for a multiscalar multiplication (msm) of edwards points.
    /// The total cost is calculated as `msm_base_cost + (length - 1) * msm_incremental_cost`.
    </span><span class="kw">pub </span>curve25519_edwards_msm_base_cost: u64,
    <span class="doccomment">/// Number of compute units consumed for a multiscalar multiplication (msm) of edwards points.
    /// The total cost is calculated as `msm_base_cost + (length - 1) * msm_incremental_cost`.
    </span><span class="kw">pub </span>curve25519_edwards_msm_incremental_cost: u64,
    <span class="doccomment">/// Number of compute units consumed to validate a curve25519 ristretto point
    </span><span class="kw">pub </span>curve25519_ristretto_validate_point_cost: u64,
    <span class="doccomment">/// Number of compute units consumed to add two curve25519 ristretto points
    </span><span class="kw">pub </span>curve25519_ristretto_add_cost: u64,
    <span class="doccomment">/// Number of compute units consumed to subtract two curve25519 ristretto points
    </span><span class="kw">pub </span>curve25519_ristretto_subtract_cost: u64,
    <span class="doccomment">/// Number of compute units consumed to multiply a curve25519 ristretto point
    </span><span class="kw">pub </span>curve25519_ristretto_multiply_cost: u64,
    <span class="doccomment">/// Number of compute units consumed for a multiscalar multiplication (msm) of ristretto points.
    /// The total cost is calculated as `msm_base_cost + (length - 1) * msm_incremental_cost`.
    </span><span class="kw">pub </span>curve25519_ristretto_msm_base_cost: u64,
    <span class="doccomment">/// Number of compute units consumed for a multiscalar multiplication (msm) of ristretto points.
    /// The total cost is calculated as `msm_base_cost + (length - 1) * msm_incremental_cost`.
    </span><span class="kw">pub </span>curve25519_ristretto_msm_incremental_cost: u64,
    <span class="doccomment">/// program heap region size, default: solana_sdk::entrypoint::HEAP_LENGTH
    </span><span class="kw">pub </span>heap_size: u32,
    <span class="doccomment">/// Number of compute units per additional 32k heap above the default (~.5
    /// us per 32k at 15 units/us rounded up)
    </span><span class="kw">pub </span>heap_cost: u64,
    <span class="doccomment">/// Memory operation syscall base cost
    </span><span class="kw">pub </span>mem_op_base_cost: u64,
    <span class="doccomment">/// Number of compute units consumed to call alt_bn128_addition
    </span><span class="kw">pub </span>alt_bn128_addition_cost: u64,
    <span class="doccomment">/// Number of compute units consumed to call alt_bn128_multiplication.
    </span><span class="kw">pub </span>alt_bn128_multiplication_cost: u64,
    <span class="doccomment">/// Total cost will be alt_bn128_pairing_one_pair_cost_first
    /// + alt_bn128_pairing_one_pair_cost_other * (num_elems - 1)
    </span><span class="kw">pub </span>alt_bn128_pairing_one_pair_cost_first: u64,
    <span class="kw">pub </span>alt_bn128_pairing_one_pair_cost_other: u64,
    <span class="doccomment">/// Big integer modular exponentiation cost
    </span><span class="kw">pub </span>big_modular_exponentiation_cost: u64,
    <span class="doccomment">/// Coefficient `a` of the quadratic function which determines the number
    /// of compute units consumed to call poseidon syscall for a given number
    /// of inputs.
    </span><span class="kw">pub </span>poseidon_cost_coefficient_a: u64,
    <span class="doccomment">/// Coefficient `c` of the quadratic function which determines the number
    /// of compute units consumed to call poseidon syscall for a given number
    /// of inputs.
    </span><span class="kw">pub </span>poseidon_cost_coefficient_c: u64,
    <span class="doccomment">/// Number of compute units consumed for accessing the remaining compute units.
    </span><span class="kw">pub </span>get_remaining_compute_units_cost: u64,
    <span class="doccomment">/// Number of compute units consumed to call alt_bn128_g1_compress.
    </span><span class="kw">pub </span>alt_bn128_g1_compress: u64,
    <span class="doccomment">/// Number of compute units consumed to call alt_bn128_g1_decompress.
    </span><span class="kw">pub </span>alt_bn128_g1_decompress: u64,
    <span class="doccomment">/// Number of compute units consumed to call alt_bn128_g2_compress.
    </span><span class="kw">pub </span>alt_bn128_g2_compress: u64,
    <span class="doccomment">/// Number of compute units consumed to call alt_bn128_g2_decompress.
    </span><span class="kw">pub </span>alt_bn128_g2_decompress: u64,
}

<span class="kw">impl </span>Default <span class="kw">for </span>ComputeBudget {
    <span class="kw">fn </span>default() -&gt; <span class="self">Self </span>{
        <span class="self">Self</span>::new(compute_budget_processor::MAX_COMPUTE_UNIT_LIMIT <span class="kw">as </span>u64)
    }
}

<span class="kw">impl </span>ComputeBudget {
    <span class="kw">pub fn </span>new(compute_unit_limit: u64) -&gt; <span class="self">Self </span>{
        ComputeBudget {
            compute_unit_limit,
            log_64_units: <span class="number">100</span>,
            create_program_address_units: <span class="number">1500</span>,
            invoke_units: <span class="number">1000</span>,
            max_invoke_stack_height: <span class="number">5</span>,
            max_instruction_trace_length: <span class="number">64</span>,
            sha256_base_cost: <span class="number">85</span>,
            sha256_byte_cost: <span class="number">1</span>,
            sha256_max_slices: <span class="number">20_000</span>,
            max_call_depth: <span class="number">64</span>,
            stack_frame_size: <span class="number">4_096</span>,
            log_pubkey_units: <span class="number">100</span>,
            max_cpi_instruction_size: <span class="number">1280</span>, <span class="comment">// IPv6 Min MTU size
            </span>cpi_bytes_per_unit: <span class="number">250</span>,        <span class="comment">// ~50MB at 200,000 units
            </span>sysvar_base_cost: <span class="number">100</span>,
            secp256k1_recover_cost: <span class="number">25_000</span>,
            syscall_base_cost: <span class="number">100</span>,
            curve25519_edwards_validate_point_cost: <span class="number">159</span>,
            curve25519_edwards_add_cost: <span class="number">473</span>,
            curve25519_edwards_subtract_cost: <span class="number">475</span>,
            curve25519_edwards_multiply_cost: <span class="number">2_177</span>,
            curve25519_edwards_msm_base_cost: <span class="number">2_273</span>,
            curve25519_edwards_msm_incremental_cost: <span class="number">758</span>,
            curve25519_ristretto_validate_point_cost: <span class="number">169</span>,
            curve25519_ristretto_add_cost: <span class="number">521</span>,
            curve25519_ristretto_subtract_cost: <span class="number">519</span>,
            curve25519_ristretto_multiply_cost: <span class="number">2_208</span>,
            curve25519_ristretto_msm_base_cost: <span class="number">2303</span>,
            curve25519_ristretto_msm_incremental_cost: <span class="number">788</span>,
            heap_size: u32::try_from(solana_sdk::entrypoint::HEAP_LENGTH).unwrap(),
            heap_cost: DEFAULT_HEAP_COST,
            mem_op_base_cost: <span class="number">10</span>,
            alt_bn128_addition_cost: <span class="number">334</span>,
            alt_bn128_multiplication_cost: <span class="number">3_840</span>,
            alt_bn128_pairing_one_pair_cost_first: <span class="number">36_364</span>,
            alt_bn128_pairing_one_pair_cost_other: <span class="number">12_121</span>,
            big_modular_exponentiation_cost: <span class="number">33</span>,
            poseidon_cost_coefficient_a: <span class="number">61</span>,
            poseidon_cost_coefficient_c: <span class="number">542</span>,
            get_remaining_compute_units_cost: <span class="number">100</span>,
            alt_bn128_g1_compress: <span class="number">30</span>,
            alt_bn128_g1_decompress: <span class="number">398</span>,
            alt_bn128_g2_compress: <span class="number">86</span>,
            alt_bn128_g2_decompress: <span class="number">13610</span>,
        }
    }

    <span class="kw">pub fn </span>try_from_instructions&lt;<span class="lifetime">'a</span>&gt;(
        instructions: <span class="kw">impl </span>Iterator&lt;Item = (<span class="kw-2">&amp;</span><span class="lifetime">'a </span>Pubkey, <span class="kw-2">&amp;</span><span class="lifetime">'a </span>CompiledInstruction)&gt;,
    ) -&gt; <span class="prelude-ty">Result</span>&lt;<span class="self">Self</span>&gt; {
        <span class="kw">let </span>compute_budget_limits = process_compute_budget_instructions(instructions)<span class="question-mark">?</span>;
        <span class="prelude-val">Ok</span>(ComputeBudget {
            compute_unit_limit: u64::from(compute_budget_limits.compute_unit_limit),
            heap_size: compute_budget_limits.updated_heap_bytes,
            ..ComputeBudget::default()
        })
    }

    <span class="doccomment">/// Returns cost of the Poseidon hash function for the given number of
    /// inputs is determined by the following quadratic function:
    ///
    /// 61*n^2 + 542
    ///
    /// Which aproximates the results of benchmarks of light-posiedon
    /// library[0]. These results assume 1 CU per 33 ns. Examples:
    ///
    /// * 1 input
    ///   * light-poseidon benchmark: `18,303 / 33 ≈ 555`
    ///   * function: `61*1^2 + 542 = 603`
    /// * 2 inputs
    ///   * light-poseidon benchmark: `25,866 / 33 ≈ 784`
    ///   * function: `61*2^2 + 542 = 786`
    /// * 3 inputs
    ///   * light-poseidon benchmark: `37,549 / 33 ≈ 1,138`
    ///   * function; `61*3^2 + 542 = 1091`
    ///
    /// [0] https://github.com/Lightprotocol/light-poseidon#performance
    </span><span class="kw">pub fn </span>poseidon_cost(<span class="kw-2">&amp;</span><span class="self">self</span>, nr_inputs: u64) -&gt; <span class="prelude-ty">Option</span>&lt;u64&gt; {
        <span class="kw">let </span>squared_inputs = nr_inputs.checked_pow(<span class="number">2</span>)<span class="question-mark">?</span>;
        <span class="kw">let </span>mul_result = <span class="self">self
            </span>.poseidon_cost_coefficient_a
            .checked_mul(squared_inputs)<span class="question-mark">?</span>;
        <span class="kw">let </span>final_result = mul_result.checked_add(<span class="self">self</span>.poseidon_cost_coefficient_c)<span class="question-mark">?</span>;

        <span class="prelude-val">Some</span>(final_result)
    }
}
</code></pre></div></section></main></body></html>