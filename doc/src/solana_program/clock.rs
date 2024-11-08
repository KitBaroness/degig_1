<!DOCTYPE html><html lang="en"><head><meta charset="utf-8"><meta name="viewport" content="width=device-width, initial-scale=1.0"><meta name="generator" content="rustdoc"><meta name="description" content="Source of the Rust file `/Users/jeyakumar/.cargo/registry/src/index.crates.io-6f17d22bba15001f/solana-program-1.18.9/src/clock.rs`."><title>clock.rs - source</title><script>if(window.location.protocol!=="file:")document.head.insertAdjacentHTML("beforeend","SourceSerif4-Regular-46f98efaafac5295.ttf.woff2,FiraSans-Regular-018c141bf0843ffd.woff2,FiraSans-Medium-8f9a781e4970d388.woff2,SourceCodePro-Regular-562dcc5011b6de7d.ttf.woff2,SourceCodePro-Semibold-d899c5a5c4aeb14a.ttf.woff2".split(",").map(f=>`<link rel="preload" as="font" type="font/woff2" crossorigin href="../../static.files/${f}">`).join(""))</script><link rel="stylesheet" href="../../static.files/normalize-76eba96aa4d2e634.css"><link rel="stylesheet" href="../../static.files/rustdoc-081576b923113409.css"><meta name="rustdoc-vars" data-root-path="../../" data-static-root-path="../../static.files/" data-current-crate="solana_program" data-themes="" data-resource-suffix="" data-rustdoc-version="1.79.0 (129f3b996 2024-06-10)" data-channel="1.79.0" data-search-js="search-bf21c90c8c1d92b1.js" data-settings-js="settings-4313503d2e1961c2.js" ><script src="../../static.files/storage-e32f0c247825364d.js"></script><script defer src="../../static.files/src-script-e66d777a5a92e9b2.js"></script><script defer src="../../src-files.js"></script><script defer src="../../static.files/main-20a3ad099b048cf2.js"></script><noscript><link rel="stylesheet" href="../../static.files/noscript-09095024cf37855e.css"></noscript><link rel="alternate icon" type="image/png" href="../../static.files/favicon-32x32-422f7d1d52889060.png"><link rel="icon" type="image/svg+xml" href="../../static.files/favicon-2c020d218678b618.svg"></head><body class="rustdoc src"><!--[if lte IE 11]><div class="warning">This old browser is unsupported and will most likely display funky things.</div><![endif]--><nav class="sidebar"><div class="src-sidebar-title"><h2>Files</h2></div></nav><div class="sidebar-resizer"></div><main><nav class="sub"><form class="search-form"><span></span><div id="sidebar-button" tabindex="-1"><a href="../../solana_program/all.html" title="show sidebar"></a></div><input class="search-input" name="search" aria-label="Run search in the documentation" autocomplete="off" spellcheck="false" placeholder="Type ‘S’ or ‘/’ to search, ‘?’ for more options…" type="search"><div id="help-button" tabindex="-1"><a href="../../help.html" title="help">?</a></div><div id="settings-menu" tabindex="-1"><a href="../../settings.html" title="settings">Settings</a></div></form></nav><section id="main-content" class="content"><div class="example-wrap"><div data-nosnippet><pre class="src-line-numbers"><a href="#1" id="1">1</a>
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
</pre></div><pre class="rust"><code><span class="doccomment">//! Information about the network's clock, ticks, slots, etc.
//!
//! Time in Solana is marked primarily by _slots_, which occur approximately every
//! 400 milliseconds, and are numbered sequentially. For every slot, a leader is
//! chosen from the validator set, and that leader is expected to produce a new
//! block, though sometimes leaders may fail to do so. Blocks can be identified
//! by their slot number, and some slots do not contain a block.
//!
//! An approximation of the passage of real-world time can be calculated by
//! multiplying a number of slots by [`DEFAULT_MS_PER_SLOT`], which is a constant target
//! time for the network to produce slots. Note though that this method suffers
//! a variable amount of drift, as the network does not produce slots at exactly
//! the target rate, and the greater number of slots being calculated for, the
//! greater the drift. Epochs cannot be used this way as they contain variable
//! numbers of slots.
//!
//! The network's current view of the real-world time can always be accessed via
//! [`Clock::unix_timestamp`], which is produced by an [oracle derived from the
//! validator set][oracle].
//!
//! [oracle]: https://docs.solanalabs.com/implemented-proposals/validator-timestamp-oracle

</span><span class="kw">use </span>solana_sdk_macro::CloneZeroed;

<span class="doccomment">/// The default tick rate that the cluster attempts to achieve (160 per second).
///
/// Note that the actual tick rate at any given time should be expected to drift.
</span><span class="kw">pub const </span>DEFAULT_TICKS_PER_SECOND: u64 = <span class="number">160</span>;

<span class="attr">#[cfg(test)]
</span><span class="macro">static_assertions::const_assert_eq!</span>(MS_PER_TICK, <span class="number">6</span>);

<span class="doccomment">/// The number of milliseconds per tick (6).
</span><span class="kw">pub const </span>MS_PER_TICK: u64 = <span class="number">1000 </span>/ DEFAULT_TICKS_PER_SECOND;

<span class="attr">#[deprecated(since = <span class="string">"1.15.0"</span>, note = <span class="string">"Please use DEFAULT_MS_PER_SLOT instead"</span>)]
</span><span class="doccomment">/// The expected duration of a slot (400 milliseconds).
</span><span class="kw">pub const </span>SLOT_MS: u64 = DEFAULT_MS_PER_SLOT;

<span class="comment">// At 160 ticks/s, 64 ticks per slot implies that leader rotation and voting will happen
// every 400 ms. A fast voting cadence ensures faster finality and convergence
</span><span class="kw">pub const </span>DEFAULT_TICKS_PER_SLOT: u64 = <span class="number">64</span>;

<span class="comment">// GCP n1-standard hardware and also a xeon e5-2520 v4 are about this rate of hashes/s
</span><span class="kw">pub const </span>DEFAULT_HASHES_PER_SECOND: u64 = <span class="number">2_000_000</span>;

<span class="comment">// Empirical sampling of mainnet validator hash rate showed the following stake
// percentages can exceed the designated hash rates as of July 2023:
// 97.6%
</span><span class="kw">pub const </span>UPDATED_HASHES_PER_SECOND_2: u64 = <span class="number">2_800_000</span>;
<span class="comment">// 96.2%
</span><span class="kw">pub const </span>UPDATED_HASHES_PER_SECOND_3: u64 = <span class="number">4_400_000</span>;
<span class="comment">// 96.2%
</span><span class="kw">pub const </span>UPDATED_HASHES_PER_SECOND_4: u64 = <span class="number">7_600_000</span>;
<span class="comment">// 96.2%
</span><span class="kw">pub const </span>UPDATED_HASHES_PER_SECOND_5: u64 = <span class="number">9_200_000</span>;
<span class="comment">// 96.2%
</span><span class="kw">pub const </span>UPDATED_HASHES_PER_SECOND_6: u64 = <span class="number">10_000_000</span>;

<span class="attr">#[cfg(test)]
</span><span class="macro">static_assertions::const_assert_eq!</span>(DEFAULT_HASHES_PER_TICK, <span class="number">12_500</span>);
<span class="kw">pub const </span>DEFAULT_HASHES_PER_TICK: u64 = DEFAULT_HASHES_PER_SECOND / DEFAULT_TICKS_PER_SECOND;

<span class="attr">#[cfg(test)]
</span><span class="macro">static_assertions::const_assert_eq!</span>(UPDATED_HASHES_PER_TICK2, <span class="number">17_500</span>);
<span class="kw">pub const </span>UPDATED_HASHES_PER_TICK2: u64 = UPDATED_HASHES_PER_SECOND_2 / DEFAULT_TICKS_PER_SECOND;

<span class="attr">#[cfg(test)]
</span><span class="macro">static_assertions::const_assert_eq!</span>(UPDATED_HASHES_PER_TICK3, <span class="number">27_500</span>);
<span class="kw">pub const </span>UPDATED_HASHES_PER_TICK3: u64 = UPDATED_HASHES_PER_SECOND_3 / DEFAULT_TICKS_PER_SECOND;

<span class="attr">#[cfg(test)]
</span><span class="macro">static_assertions::const_assert_eq!</span>(UPDATED_HASHES_PER_TICK4, <span class="number">47_500</span>);
<span class="kw">pub const </span>UPDATED_HASHES_PER_TICK4: u64 = UPDATED_HASHES_PER_SECOND_4 / DEFAULT_TICKS_PER_SECOND;

<span class="attr">#[cfg(test)]
</span><span class="macro">static_assertions::const_assert_eq!</span>(UPDATED_HASHES_PER_TICK5, <span class="number">57_500</span>);
<span class="kw">pub const </span>UPDATED_HASHES_PER_TICK5: u64 = UPDATED_HASHES_PER_SECOND_5 / DEFAULT_TICKS_PER_SECOND;

<span class="attr">#[cfg(test)]
</span><span class="macro">static_assertions::const_assert_eq!</span>(UPDATED_HASHES_PER_TICK6, <span class="number">62_500</span>);
<span class="kw">pub const </span>UPDATED_HASHES_PER_TICK6: u64 = UPDATED_HASHES_PER_SECOND_6 / DEFAULT_TICKS_PER_SECOND;

<span class="comment">// 1 Dev Epoch = 400 ms * 8192 ~= 55 minutes
</span><span class="kw">pub const </span>DEFAULT_DEV_SLOTS_PER_EPOCH: u64 = <span class="number">8192</span>;

<span class="attr">#[cfg(test)]
</span><span class="macro">static_assertions::const_assert_eq!</span>(SECONDS_PER_DAY, <span class="number">86_400</span>);
<span class="kw">pub const </span>SECONDS_PER_DAY: u64 = <span class="number">24 </span>* <span class="number">60 </span>* <span class="number">60</span>;

<span class="attr">#[cfg(test)]
</span><span class="macro">static_assertions::const_assert_eq!</span>(TICKS_PER_DAY, <span class="number">13_824_000</span>);
<span class="kw">pub const </span>TICKS_PER_DAY: u64 = DEFAULT_TICKS_PER_SECOND * SECONDS_PER_DAY;

<span class="attr">#[cfg(test)]
</span><span class="macro">static_assertions::const_assert_eq!</span>(DEFAULT_SLOTS_PER_EPOCH, <span class="number">432_000</span>);

<span class="doccomment">/// The number of slots per epoch after initial network warmup.
///
/// 1 Epoch ~= 2 days.
</span><span class="kw">pub const </span>DEFAULT_SLOTS_PER_EPOCH: u64 = <span class="number">2 </span>* TICKS_PER_DAY / DEFAULT_TICKS_PER_SLOT;

<span class="comment">// leader schedule is governed by this
</span><span class="kw">pub const </span>NUM_CONSECUTIVE_LEADER_SLOTS: u64 = <span class="number">4</span>;

<span class="attr">#[cfg(test)]
</span><span class="macro">static_assertions::const_assert_eq!</span>(DEFAULT_MS_PER_SLOT, <span class="number">400</span>);
<span class="doccomment">/// The expected duration of a slot (400 milliseconds).
</span><span class="kw">pub const </span>DEFAULT_MS_PER_SLOT: u64 = <span class="number">1_000 </span>* DEFAULT_TICKS_PER_SLOT / DEFAULT_TICKS_PER_SECOND;
<span class="kw">pub const </span>DEFAULT_S_PER_SLOT: f64 = DEFAULT_TICKS_PER_SLOT <span class="kw">as </span>f64 / DEFAULT_TICKS_PER_SECOND <span class="kw">as </span>f64;

<span class="doccomment">/// The time window of recent block hash values over which the bank will track
/// signatures.
///
/// Once the bank discards a block hash, it will reject any transactions that
/// use that `recent_blockhash` in a transaction. Lowering this value reduces
/// memory consumption, but requires a client to update its `recent_blockhash`
/// more frequently. Raising the value lengthens the time a client must wait to
/// be certain a missing transaction will not be processed by the network.
</span><span class="kw">pub const </span>MAX_HASH_AGE_IN_SECONDS: usize = <span class="number">120</span>;

<span class="attr">#[cfg(test)]
</span><span class="macro">static_assertions::const_assert_eq!</span>(MAX_RECENT_BLOCKHASHES, <span class="number">300</span>);
<span class="comment">// Number of maximum recent blockhashes (one blockhash per non-skipped slot)
</span><span class="kw">pub const </span>MAX_RECENT_BLOCKHASHES: usize =
    MAX_HASH_AGE_IN_SECONDS * DEFAULT_TICKS_PER_SECOND <span class="kw">as </span>usize / DEFAULT_TICKS_PER_SLOT <span class="kw">as </span>usize;

<span class="attr">#[cfg(test)]
</span><span class="macro">static_assertions::const_assert_eq!</span>(MAX_PROCESSING_AGE, <span class="number">150</span>);
<span class="comment">// The maximum age of a blockhash that will be accepted by the leader
</span><span class="kw">pub const </span>MAX_PROCESSING_AGE: usize = MAX_RECENT_BLOCKHASHES / <span class="number">2</span>;

<span class="doccomment">/// This is maximum time consumed in forwarding a transaction from one node to next, before
/// it can be processed in the target node
</span><span class="kw">pub const </span>MAX_TRANSACTION_FORWARDING_DELAY_GPU: usize = <span class="number">2</span>;

<span class="doccomment">/// More delay is expected if CUDA is not enabled (as signature verification takes longer)
</span><span class="kw">pub const </span>MAX_TRANSACTION_FORWARDING_DELAY: usize = <span class="number">6</span>;

<span class="doccomment">/// Transaction forwarding, which leader to forward to and how long to hold
</span><span class="kw">pub const </span>FORWARD_TRANSACTIONS_TO_LEADER_AT_SLOT_OFFSET: u64 = <span class="number">2</span>;
<span class="kw">pub const </span>HOLD_TRANSACTIONS_SLOT_OFFSET: u64 = <span class="number">20</span>;

<span class="doccomment">/// The unit of time given to a leader for encoding a block.
///
/// It is some some number of _ticks_ long.
</span><span class="kw">pub type </span>Slot = u64;

<span class="doccomment">/// Uniquely distinguishes every version of a slot.
///
/// The `BankId` is unique even if the slot number of two different slots is the
/// same. This can happen in the case of e.g. duplicate slots.
</span><span class="kw">pub type </span>BankId = u64;

<span class="doccomment">/// The unit of time a given leader schedule is honored.
///
/// It lasts for some number of [`Slot`]s.
</span><span class="kw">pub type </span>Epoch = u64;

<span class="kw">pub const </span>GENESIS_EPOCH: Epoch = <span class="number">0</span>;
<span class="comment">// must be sync with Account::rent_epoch::default()
</span><span class="kw">pub const </span>INITIAL_RENT_EPOCH: Epoch = <span class="number">0</span>;

<span class="doccomment">/// An index to the slots of a epoch.
</span><span class="kw">pub type </span>SlotIndex = u64;

<span class="doccomment">/// The number of slots in a epoch.
</span><span class="kw">pub type </span>SlotCount = u64;

<span class="doccomment">/// An approximate measure of real-world time.
///
/// Expressed as Unix time (i.e. seconds since the Unix epoch).
</span><span class="kw">pub type </span>UnixTimestamp = i64;

<span class="doccomment">/// A representation of network time.
///
/// All members of `Clock` start from 0 upon network boot.
</span><span class="attr">#[repr(C)]
#[derive(Serialize, Deserialize, Debug, CloneZeroed, Default, PartialEq, Eq)]
</span><span class="kw">pub struct </span>Clock {
    <span class="doccomment">/// The current `Slot`.
    </span><span class="kw">pub </span>slot: Slot,
    <span class="doccomment">/// The timestamp of the first `Slot` in this `Epoch`.
    </span><span class="kw">pub </span>epoch_start_timestamp: UnixTimestamp,
    <span class="doccomment">/// The current `Epoch`.
    </span><span class="kw">pub </span>epoch: Epoch,
    <span class="doccomment">/// The future `Epoch` for which the leader schedule has
    /// most recently been calculated.
    </span><span class="kw">pub </span>leader_schedule_epoch: Epoch,
    <span class="doccomment">/// The approximate real world time of the current slot.
    ///
    /// This value was originally computed from genesis creation time and
    /// network time in slots, incurring a lot of drift. Following activation of
    /// the [`timestamp_correction` and `timestamp_bounding`][tsc] features it
    /// is calculated using a [validator timestamp oracle][oracle].
    ///
    /// [tsc]: https://docs.solanalabs.com/implemented-proposals/bank-timestamp-correction
    /// [oracle]: https://docs.solanalabs.com/implemented-proposals/validator-timestamp-oracle
    </span><span class="kw">pub </span>unix_timestamp: UnixTimestamp,
}

<span class="attr">#[cfg(test)]
</span><span class="kw">mod </span>tests {
    <span class="kw">use super</span>::<span class="kw-2">*</span>;

    <span class="attr">#[test]
    </span><span class="kw">fn </span>test_clone() {
        <span class="kw">let </span>clock = Clock {
            slot: <span class="number">1</span>,
            epoch_start_timestamp: <span class="number">2</span>,
            epoch: <span class="number">3</span>,
            leader_schedule_epoch: <span class="number">4</span>,
            unix_timestamp: <span class="number">5</span>,
        };
        <span class="kw">let </span>cloned_clock = clock.clone();
        <span class="macro">assert_eq!</span>(cloned_clock, clock);
    }
}
</code></pre></div></section></main></body></html>