<!DOCTYPE html><html lang="en"><head><meta charset="utf-8"><meta name="viewport" content="width=device-width, initial-scale=1.0"><meta name="generator" content="rustdoc"><meta name="description" content="Source of the Rust file `/Users/jeyakumar/.cargo/registry/src/index.crates.io-6f17d22bba15001f/anchor-lang-0.30.0/src/accounts/interface_account.rs`."><title>interface_account.rs - source</title><script>if(window.location.protocol!=="file:")document.head.insertAdjacentHTML("beforeend","SourceSerif4-Regular-46f98efaafac5295.ttf.woff2,FiraSans-Regular-018c141bf0843ffd.woff2,FiraSans-Medium-8f9a781e4970d388.woff2,SourceCodePro-Regular-562dcc5011b6de7d.ttf.woff2,SourceCodePro-Semibold-d899c5a5c4aeb14a.ttf.woff2".split(",").map(f=>`<link rel="preload" as="font" type="font/woff2" crossorigin href="../../../static.files/${f}">`).join(""))</script><link rel="stylesheet" href="../../../static.files/normalize-76eba96aa4d2e634.css"><link rel="stylesheet" href="../../../static.files/rustdoc-081576b923113409.css"><meta name="rustdoc-vars" data-root-path="../../../" data-static-root-path="../../../static.files/" data-current-crate="anchor_lang" data-themes="" data-resource-suffix="" data-rustdoc-version="1.79.0 (129f3b996 2024-06-10)" data-channel="1.79.0" data-search-js="search-bf21c90c8c1d92b1.js" data-settings-js="settings-4313503d2e1961c2.js" ><script src="../../../static.files/storage-e32f0c247825364d.js"></script><script defer src="../../../static.files/src-script-e66d777a5a92e9b2.js"></script><script defer src="../../../src-files.js"></script><script defer src="../../../static.files/main-20a3ad099b048cf2.js"></script><noscript><link rel="stylesheet" href="../../../static.files/noscript-09095024cf37855e.css"></noscript><link rel="alternate icon" type="image/png" href="../../../static.files/favicon-32x32-422f7d1d52889060.png"><link rel="icon" type="image/svg+xml" href="../../../static.files/favicon-2c020d218678b618.svg"></head><body class="rustdoc src"><!--[if lte IE 11]><div class="warning">This old browser is unsupported and will most likely display funky things.</div><![endif]--><nav class="sidebar"><div class="src-sidebar-title"><h2>Files</h2></div></nav><div class="sidebar-resizer"></div><main><nav class="sub"><form class="search-form"><span></span><div id="sidebar-button" tabindex="-1"><a href="../../../anchor_lang/all.html" title="show sidebar"></a></div><input class="search-input" name="search" aria-label="Run search in the documentation" autocomplete="off" spellcheck="false" placeholder="Type ‘S’ or ‘/’ to search, ‘?’ for more options…" type="search"><div id="help-button" tabindex="-1"><a href="../../../help.html" title="help">?</a></div><div id="settings-menu" tabindex="-1"><a href="../../../settings.html" title="settings">Settings</a></div></form></nav><section id="main-content" class="content"><div class="example-wrap"><div data-nosnippet><pre class="src-line-numbers"><a href="#1" id="1">1</a>
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
<a href="#280" id="280">280</a>
<a href="#281" id="281">281</a>
<a href="#282" id="282">282</a>
<a href="#283" id="283">283</a>
<a href="#284" id="284">284</a>
<a href="#285" id="285">285</a>
<a href="#286" id="286">286</a>
<a href="#287" id="287">287</a>
<a href="#288" id="288">288</a>
<a href="#289" id="289">289</a>
<a href="#290" id="290">290</a>
<a href="#291" id="291">291</a>
<a href="#292" id="292">292</a>
<a href="#293" id="293">293</a>
<a href="#294" id="294">294</a>
<a href="#295" id="295">295</a>
<a href="#296" id="296">296</a>
<a href="#297" id="297">297</a>
<a href="#298" id="298">298</a>
<a href="#299" id="299">299</a>
<a href="#300" id="300">300</a>
<a href="#301" id="301">301</a>
<a href="#302" id="302">302</a>
<a href="#303" id="303">303</a>
<a href="#304" id="304">304</a>
<a href="#305" id="305">305</a>
<a href="#306" id="306">306</a>
<a href="#307" id="307">307</a>
<a href="#308" id="308">308</a>
<a href="#309" id="309">309</a>
<a href="#310" id="310">310</a>
<a href="#311" id="311">311</a>
<a href="#312" id="312">312</a>
<a href="#313" id="313">313</a>
<a href="#314" id="314">314</a>
<a href="#315" id="315">315</a>
<a href="#316" id="316">316</a>
<a href="#317" id="317">317</a>
<a href="#318" id="318">318</a>
<a href="#319" id="319">319</a>
<a href="#320" id="320">320</a>
<a href="#321" id="321">321</a>
<a href="#322" id="322">322</a>
<a href="#323" id="323">323</a>
<a href="#324" id="324">324</a>
<a href="#325" id="325">325</a>
<a href="#326" id="326">326</a>
<a href="#327" id="327">327</a>
<a href="#328" id="328">328</a>
</pre></div><pre class="rust"><code><span class="doccomment">//! Account container that checks ownership on deserialization.

</span><span class="kw">use </span><span class="kw">crate</span>::accounts::account::Account;
<span class="kw">use </span><span class="kw">crate</span>::error::ErrorCode;
<span class="kw">use crate</span>::{
    AccountDeserialize, AccountSerialize, Accounts, AccountsClose, AccountsExit, CheckOwner, Key,
    Owners, <span class="prelude-ty">Result</span>, ToAccountInfos, ToAccountMetas,
};
<span class="kw">use </span>solana_program::account_info::AccountInfo;
<span class="kw">use </span>solana_program::instruction::AccountMeta;
<span class="kw">use </span>solana_program::pubkey::Pubkey;
<span class="kw">use </span>solana_program::system_program;
<span class="kw">use </span>std::collections::BTreeSet;
<span class="kw">use </span>std::fmt;
<span class="kw">use </span>std::ops::{Deref, DerefMut};

<span class="doccomment">/// Wrapper around [`AccountInfo`](crate::solana_program::account_info::AccountInfo)
/// that verifies program ownership and deserializes underlying data into a Rust type.
///
/// # Table of Contents
/// - [Basic Functionality](#basic-functionality)
/// - [Using InterfaceAccount with non-anchor types](#using-interface-account-with-non-anchor-types)
/// - [Out of the box wrapper types](#out-of-the-box-wrapper-types)
///
/// # Basic Functionality
///
/// InterfaceAccount checks that `T::owners().contains(Account.info.owner)`.
/// This means that the data type that Accounts wraps around (`=T`) needs to
/// implement the [Owners trait](crate::Owners).
/// The `#[account]` attribute implements the Owners trait for
/// a struct using multiple `crate::ID`s declared by [`declareId`](crate::declare_id)
/// in the same program. It follows that InterfaceAccount can also be used
/// with a `T` that comes from a different program.
///
/// Checks:
///
/// - `T::owners().contains(InterfaceAccount.info.owner)`
/// - `!(InterfaceAccount.info.owner == SystemProgram &amp;&amp; InterfaceAccount.info.lamports() == 0)`
///
/// # Example
/// ```ignore
/// use anchor_lang::prelude::*;
/// use other_program::Auth;
///
/// declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");
///
/// #[program]
/// mod hello_anchor {
///     use super::*;
///     pub fn set_data(ctx: Context&lt;SetData&gt;, data: u64) -&gt; Result&lt;()&gt; {
///         if (*ctx.accounts.auth_account).authorized {
///             (*ctx.accounts.my_account).data = data;
///         }
///         Ok(())
///     }
/// }
///
/// #[account]
/// #[derive(Default)]
/// pub struct MyData {
///     pub data: u64
/// }
///
/// #[derive(Accounts)]
/// pub struct SetData&lt;'info&gt; {
///     #[account(mut)]
///     pub my_account: InterfaceAccount&lt;'info, MyData&gt; // checks that my_account.info.owner == Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS
///     pub auth_account: InterfaceAccount&lt;'info, Auth&gt; // checks that auth_account.info.owner == FEZGUxNhZWpYPj9MJCrZJvUo1iF9ys34UHx52y4SzVW9
/// }
///
/// // In a different program
///
/// ...
/// declare_id!("FEZGUxNhZWpYPj9MJCrZJvUo1iF9ys34UHx52y4SzVW9");
/// #[account]
/// #[derive(Default)]
/// pub struct Auth {
///     pub authorized: bool
/// }
/// ...
/// ```
///
/// # Using InterfaceAccount with non-anchor programs
///
/// InterfaceAccount can also be used with non-anchor programs. The data types from
/// those programs are not annotated with `#[account]` so you have to
/// - create a wrapper type around the structs you want to wrap with InterfaceAccount
/// - implement the functions required by InterfaceAccount yourself
/// instead of using `#[account]`. You only have to implement a fraction of the
/// functions `#[account]` generates. See the example below for the code you have
/// to write.
///
/// The mint wrapper type that Anchor provides out of the box for the token program ([source](https://github.com/coral-xyz/anchor/blob/master/spl/src/token.rs))
/// ```ignore
/// #[derive(Clone)]
/// pub struct Mint(spl_token::state::Mint);
///
/// // This is necessary so we can use "anchor_spl::token::Mint::LEN"
/// // because rust does not resolve "anchor_spl::token::Mint::LEN" to
/// // "spl_token::state::Mint::LEN" automatically
/// impl Mint {
///     pub const LEN: usize = spl_token::state::Mint::LEN;
/// }
///
/// // You don't have to implement the "try_deserialize" function
/// // from this trait. It delegates to
/// // "try_deserialize_unchecked" by default which is what we want here
/// // because non-anchor accounts don't have a discriminator to check
/// impl anchor_lang::AccountDeserialize for Mint {
///     fn try_deserialize_unchecked(buf: &amp;mut &amp;[u8]) -&gt; Result&lt;Self&gt; {
///         spl_token::state::Mint::unpack(buf).map(Mint)
///     }
/// }
/// // AccountSerialize defaults to a no-op which is what we want here
/// // because it's a foreign program, so our program does not
/// // have permission to write to the foreign program's accounts anyway
/// impl anchor_lang::AccountSerialize for Mint {}
///
/// impl anchor_lang::Owner for Mint {
///     fn owner() -&gt; Pubkey {
///         // pub use spl_token::ID is used at the top of the file
///         ID
///     }
/// }
///
/// // Implement the "std::ops::Deref" trait for better user experience
/// impl Deref for Mint {
///     type Target = spl_token::state::Mint;
///
///     fn deref(&amp;self) -&gt; &amp;Self::Target {
///         &amp;self.0
///     }
/// }
/// ```
///
/// ## Out of the box wrapper types
///
/// ### SPL Types
///
/// Anchor provides wrapper types to access accounts owned by the token programs. Use
/// ```ignore
/// use anchor_spl::token_interface::TokenAccount;
///
/// #[derive(Accounts)]
/// pub struct Example {
///     pub my_acc: InterfaceAccount&lt;'info, TokenAccount&gt;
/// }
/// ```
/// to access token accounts and
/// ```ignore
/// use anchor_spl::token_interface::Mint;
///
/// #[derive(Accounts)]
/// pub struct Example {
///     pub my_acc: InterfaceAccount&lt;'info, Mint&gt;
/// }
/// ```
/// to access mint accounts.
</span><span class="attr">#[derive(Clone)]
</span><span class="kw">pub struct </span>InterfaceAccount&lt;<span class="lifetime">'info</span>, T: AccountSerialize + AccountDeserialize + Clone&gt; {
    account: Account&lt;<span class="lifetime">'info</span>, T&gt;,
    <span class="comment">// The owner here is used to make sure that changes aren't incorrectly propagated
    // to an account with a modified owner
    </span>owner: Pubkey,
}

<span class="kw">impl</span>&lt;<span class="lifetime">'info</span>, T: AccountSerialize + AccountDeserialize + Clone + fmt::Debug&gt; fmt::Debug
    <span class="kw">for </span>InterfaceAccount&lt;<span class="lifetime">'info</span>, T&gt;
{
    <span class="kw">fn </span>fmt(<span class="kw-2">&amp;</span><span class="self">self</span>, f: <span class="kw-2">&amp;mut </span>fmt::Formatter&lt;<span class="lifetime">'_</span>&gt;) -&gt; fmt::Result {
        <span class="self">self</span>.account.fmt_with_name(<span class="string">"InterfaceAccount"</span>, f)
    }
}

<span class="kw">impl</span>&lt;<span class="lifetime">'a</span>, T: AccountSerialize + AccountDeserialize + Clone&gt; InterfaceAccount&lt;<span class="lifetime">'a</span>, T&gt; {
    <span class="kw">fn </span>new(info: <span class="kw-2">&amp;</span><span class="lifetime">'a </span>AccountInfo&lt;<span class="lifetime">'a</span>&gt;, account: T) -&gt; <span class="self">Self </span>{
        <span class="kw">let </span>owner = <span class="kw-2">*</span>info.owner;
        <span class="self">Self </span>{
            account: Account::new(info, account),
            owner,
        }
    }

    <span class="doccomment">/// Reloads the account from storage. This is useful, for example, when
    /// observing side effects after CPI.
    </span><span class="kw">pub fn </span>reload(<span class="kw-2">&amp;mut </span><span class="self">self</span>) -&gt; <span class="prelude-ty">Result</span>&lt;()&gt; {
        <span class="self">self</span>.account.reload()
    }

    <span class="kw">pub fn </span>into_inner(<span class="self">self</span>) -&gt; T {
        <span class="self">self</span>.account.into_inner()
    }

    <span class="doccomment">/// Sets the inner account.
    ///
    /// Instead of this:
    /// ```ignore
    /// pub fn new_user(ctx: Context&lt;CreateUser&gt;, new_user:User) -&gt; Result&lt;()&gt; {
    ///     (*ctx.accounts.user_to_create).name = new_user.name;
    ///     (*ctx.accounts.user_to_create).age = new_user.age;
    ///     (*ctx.accounts.user_to_create).address = new_user.address;
    /// }
    /// ```
    /// You can do this:
    /// ```ignore
    /// pub fn new_user(ctx: Context&lt;CreateUser&gt;, new_user:User) -&gt; Result&lt;()&gt; {
    ///     ctx.accounts.user_to_create.set_inner(new_user);
    /// }
    /// ```
    </span><span class="kw">pub fn </span>set_inner(<span class="kw-2">&amp;mut </span><span class="self">self</span>, inner: T) {
        <span class="self">self</span>.account.set_inner(inner);
    }
}

<span class="kw">impl</span>&lt;<span class="lifetime">'a</span>, T: AccountSerialize + AccountDeserialize + CheckOwner + Clone&gt; InterfaceAccount&lt;<span class="lifetime">'a</span>, T&gt; {
    <span class="doccomment">/// Deserializes the given `info` into a `InterfaceAccount`.
    </span><span class="attr">#[inline(never)]
    </span><span class="kw">pub fn </span>try_from(info: <span class="kw-2">&amp;</span><span class="lifetime">'a </span>AccountInfo&lt;<span class="lifetime">'a</span>&gt;) -&gt; <span class="prelude-ty">Result</span>&lt;<span class="self">Self</span>&gt; {
        <span class="kw">if </span>info.owner == <span class="kw-2">&amp;</span>system_program::ID &amp;&amp; info.lamports() == <span class="number">0 </span>{
            <span class="kw">return </span><span class="prelude-val">Err</span>(ErrorCode::AccountNotInitialized.into());
        }
        T::check_owner(info.owner)<span class="question-mark">?</span>;
        <span class="kw">let </span><span class="kw-2">mut </span>data: <span class="kw-2">&amp;</span>[u8] = <span class="kw-2">&amp;</span>info.try_borrow_data()<span class="question-mark">?</span>;
        <span class="prelude-val">Ok</span>(<span class="self">Self</span>::new(info, T::try_deserialize(<span class="kw-2">&amp;mut </span>data)<span class="question-mark">?</span>))
    }

    <span class="doccomment">/// Deserializes the given `info` into a `InterfaceAccount` without checking
    /// the account discriminator. Be careful when using this and avoid it if
    /// possible.
    </span><span class="attr">#[inline(never)]
    </span><span class="kw">pub fn </span>try_from_unchecked(info: <span class="kw-2">&amp;</span><span class="lifetime">'a </span>AccountInfo&lt;<span class="lifetime">'a</span>&gt;) -&gt; <span class="prelude-ty">Result</span>&lt;<span class="self">Self</span>&gt; {
        <span class="kw">if </span>info.owner == <span class="kw-2">&amp;</span>system_program::ID &amp;&amp; info.lamports() == <span class="number">0 </span>{
            <span class="kw">return </span><span class="prelude-val">Err</span>(ErrorCode::AccountNotInitialized.into());
        }
        T::check_owner(info.owner)<span class="question-mark">?</span>;
        <span class="kw">let </span><span class="kw-2">mut </span>data: <span class="kw-2">&amp;</span>[u8] = <span class="kw-2">&amp;</span>info.try_borrow_data()<span class="question-mark">?</span>;
        <span class="prelude-val">Ok</span>(<span class="self">Self</span>::new(info, T::try_deserialize_unchecked(<span class="kw-2">&amp;mut </span>data)<span class="question-mark">?</span>))
    }
}

<span class="kw">impl</span>&lt;<span class="lifetime">'info</span>, B, T: AccountSerialize + AccountDeserialize + CheckOwner + Clone&gt; Accounts&lt;<span class="lifetime">'info</span>, B&gt;
    <span class="kw">for </span>InterfaceAccount&lt;<span class="lifetime">'info</span>, T&gt;
{
    <span class="attr">#[inline(never)]
    </span><span class="kw">fn </span>try_accounts(
        _program_id: <span class="kw-2">&amp;</span>Pubkey,
        accounts: <span class="kw-2">&amp;mut &amp;</span><span class="lifetime">'info </span>[AccountInfo&lt;<span class="lifetime">'info</span>&gt;],
        _ix_data: <span class="kw-2">&amp;</span>[u8],
        _bumps: <span class="kw-2">&amp;mut </span>B,
        _reallocs: <span class="kw-2">&amp;mut </span>BTreeSet&lt;Pubkey&gt;,
    ) -&gt; <span class="prelude-ty">Result</span>&lt;<span class="self">Self</span>&gt; {
        <span class="kw">if </span>accounts.is_empty() {
            <span class="kw">return </span><span class="prelude-val">Err</span>(ErrorCode::AccountNotEnoughKeys.into());
        }
        <span class="kw">let </span>account = <span class="kw-2">&amp;</span>accounts[<span class="number">0</span>];
        <span class="kw-2">*</span>accounts = <span class="kw-2">&amp;</span>accounts[<span class="number">1</span>..];
        <span class="self">Self</span>::try_from(account)
    }
}

<span class="kw">impl</span>&lt;<span class="lifetime">'info</span>, T: AccountSerialize + AccountDeserialize + Owners + Clone&gt; AccountsExit&lt;<span class="lifetime">'info</span>&gt;
    <span class="kw">for </span>InterfaceAccount&lt;<span class="lifetime">'info</span>, T&gt;
{
    <span class="kw">fn </span>exit(<span class="kw-2">&amp;</span><span class="self">self</span>, program_id: <span class="kw-2">&amp;</span>Pubkey) -&gt; <span class="prelude-ty">Result</span>&lt;()&gt; {
        <span class="self">self</span>.account
            .exit_with_expected_owner(<span class="kw-2">&amp;</span><span class="self">self</span>.owner, program_id)
    }
}

<span class="kw">impl</span>&lt;<span class="lifetime">'info</span>, T: AccountSerialize + AccountDeserialize + Clone&gt; AccountsClose&lt;<span class="lifetime">'info</span>&gt;
    <span class="kw">for </span>InterfaceAccount&lt;<span class="lifetime">'info</span>, T&gt;
{
    <span class="kw">fn </span>close(<span class="kw-2">&amp;</span><span class="self">self</span>, sol_destination: AccountInfo&lt;<span class="lifetime">'info</span>&gt;) -&gt; <span class="prelude-ty">Result</span>&lt;()&gt; {
        <span class="self">self</span>.account.close(sol_destination)
    }
}

<span class="kw">impl</span>&lt;<span class="lifetime">'info</span>, T: AccountSerialize + AccountDeserialize + Clone&gt; ToAccountMetas
    <span class="kw">for </span>InterfaceAccount&lt;<span class="lifetime">'info</span>, T&gt;
{
    <span class="kw">fn </span>to_account_metas(<span class="kw-2">&amp;</span><span class="self">self</span>, is_signer: <span class="prelude-ty">Option</span>&lt;bool&gt;) -&gt; Vec&lt;AccountMeta&gt; {
        <span class="self">self</span>.account.to_account_metas(is_signer)
    }
}

<span class="kw">impl</span>&lt;<span class="lifetime">'info</span>, T: AccountSerialize + AccountDeserialize + Clone&gt; ToAccountInfos&lt;<span class="lifetime">'info</span>&gt;
    <span class="kw">for </span>InterfaceAccount&lt;<span class="lifetime">'info</span>, T&gt;
{
    <span class="kw">fn </span>to_account_infos(<span class="kw-2">&amp;</span><span class="self">self</span>) -&gt; Vec&lt;AccountInfo&lt;<span class="lifetime">'info</span>&gt;&gt; {
        <span class="self">self</span>.account.to_account_infos()
    }
}

<span class="kw">impl</span>&lt;<span class="lifetime">'info</span>, T: AccountSerialize + AccountDeserialize + Clone&gt; AsRef&lt;AccountInfo&lt;<span class="lifetime">'info</span>&gt;&gt;
    <span class="kw">for </span>InterfaceAccount&lt;<span class="lifetime">'info</span>, T&gt;
{
    <span class="kw">fn </span>as_ref(<span class="kw-2">&amp;</span><span class="self">self</span>) -&gt; <span class="kw-2">&amp;</span>AccountInfo&lt;<span class="lifetime">'info</span>&gt; {
        <span class="self">self</span>.account.as_ref()
    }
}

<span class="kw">impl</span>&lt;<span class="lifetime">'info</span>, T: AccountSerialize + AccountDeserialize + Clone&gt; AsRef&lt;T&gt;
    <span class="kw">for </span>InterfaceAccount&lt;<span class="lifetime">'info</span>, T&gt;
{
    <span class="kw">fn </span>as_ref(<span class="kw-2">&amp;</span><span class="self">self</span>) -&gt; <span class="kw-2">&amp;</span>T {
        <span class="self">self</span>.account.as_ref()
    }
}

<span class="kw">impl</span>&lt;<span class="lifetime">'a</span>, T: AccountSerialize + AccountDeserialize + Clone&gt; Deref <span class="kw">for </span>InterfaceAccount&lt;<span class="lifetime">'a</span>, T&gt; {
    <span class="kw">type </span>Target = T;

    <span class="kw">fn </span>deref(<span class="kw-2">&amp;</span><span class="self">self</span>) -&gt; <span class="kw-2">&amp;</span><span class="self">Self</span>::Target {
        <span class="self">self</span>.account.deref()
    }
}

<span class="kw">impl</span>&lt;<span class="lifetime">'a</span>, T: AccountSerialize + AccountDeserialize + Clone&gt; DerefMut <span class="kw">for </span>InterfaceAccount&lt;<span class="lifetime">'a</span>, T&gt; {
    <span class="kw">fn </span>deref_mut(<span class="kw-2">&amp;mut </span><span class="self">self</span>) -&gt; <span class="kw-2">&amp;mut </span><span class="self">Self</span>::Target {
        <span class="self">self</span>.account.deref_mut()
    }
}

<span class="kw">impl</span>&lt;<span class="lifetime">'info</span>, T: AccountSerialize + AccountDeserialize + Clone&gt; Key <span class="kw">for </span>InterfaceAccount&lt;<span class="lifetime">'info</span>, T&gt; {
    <span class="kw">fn </span>key(<span class="kw-2">&amp;</span><span class="self">self</span>) -&gt; Pubkey {
        <span class="self">self</span>.account.key()
    }
}
</code></pre></div></section></main></body></html>