<!DOCTYPE html><html lang="en"><head><meta charset="utf-8"><meta name="viewport" content="width=device-width, initial-scale=1.0"><meta name="generator" content="rustdoc"><meta name="description" content="Source of the Rust file `/Users/jeyakumar/.cargo/registry/src/index.crates.io-6f17d22bba15001f/anchor-lang-0.30.0/src/idl.rs`."><title>idl.rs - source</title><script>if(window.location.protocol!=="file:")document.head.insertAdjacentHTML("beforeend","SourceSerif4-Regular-46f98efaafac5295.ttf.woff2,FiraSans-Regular-018c141bf0843ffd.woff2,FiraSans-Medium-8f9a781e4970d388.woff2,SourceCodePro-Regular-562dcc5011b6de7d.ttf.woff2,SourceCodePro-Semibold-d899c5a5c4aeb14a.ttf.woff2".split(",").map(f=>`<link rel="preload" as="font" type="font/woff2" crossorigin href="../../static.files/${f}">`).join(""))</script><link rel="stylesheet" href="../../static.files/normalize-76eba96aa4d2e634.css"><link rel="stylesheet" href="../../static.files/rustdoc-081576b923113409.css"><meta name="rustdoc-vars" data-root-path="../../" data-static-root-path="../../static.files/" data-current-crate="anchor_lang" data-themes="" data-resource-suffix="" data-rustdoc-version="1.79.0 (129f3b996 2024-06-10)" data-channel="1.79.0" data-search-js="search-bf21c90c8c1d92b1.js" data-settings-js="settings-4313503d2e1961c2.js" ><script src="../../static.files/storage-e32f0c247825364d.js"></script><script defer src="../../static.files/src-script-e66d777a5a92e9b2.js"></script><script defer src="../../src-files.js"></script><script defer src="../../static.files/main-20a3ad099b048cf2.js"></script><noscript><link rel="stylesheet" href="../../static.files/noscript-09095024cf37855e.css"></noscript><link rel="alternate icon" type="image/png" href="../../static.files/favicon-32x32-422f7d1d52889060.png"><link rel="icon" type="image/svg+xml" href="../../static.files/favicon-2c020d218678b618.svg"></head><body class="rustdoc src"><!--[if lte IE 11]><div class="warning">This old browser is unsupported and will most likely display funky things.</div><![endif]--><nav class="sidebar"><div class="src-sidebar-title"><h2>Files</h2></div></nav><div class="sidebar-resizer"></div><main><nav class="sub"><form class="search-form"><span></span><div id="sidebar-button" tabindex="-1"><a href="../../anchor_lang/all.html" title="show sidebar"></a></div><input class="search-input" name="search" aria-label="Run search in the documentation" autocomplete="off" spellcheck="false" placeholder="Type ‘S’ or ‘/’ to search, ‘?’ for more options…" type="search"><div id="help-button" tabindex="-1"><a href="../../help.html" title="help">?</a></div><div id="settings-menu" tabindex="-1"><a href="../../settings.html" title="settings">Settings</a></div></form></nav><section id="main-content" class="content"><div class="example-wrap"><div data-nosnippet><pre class="src-line-numbers"><a href="#1" id="1">1</a>
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
</pre></div><pre class="rust"><code><span class="doccomment">//! Defines the instructions and account state used to store a program's
//! IDL on-chain at a canonical account address, which can be derived as a
//! function of nothing other than the program's ID.
//!
//! It can be upgraded in a way similar to a BPF upgradeable program. That is,
//! one may invoke the `IdlInstruction::CreateBuffer` instruction to create
//! a buffer, `IdlInstruction::Write` to write a new IDL into it, and then
//! `IdlInstruction::SetBuffer` to copy the IDL into the program's canonical
//! IDL account. In order to perform this upgrade, the buffer's `authority`
//! must match the canonical IDL account's authority.
//!
//! Because the IDL can be larger than the max transaction size, the transaction
//! must be broken up into several pieces and stored into the IDL account with
//! multiple transactions via the `Write` instruction to continuously append to
//! the account's IDL data buffer.
//!
//! Note that IDL account instructions are automatically inserted into all
//! Anchor programs. To remove them, one can use the `no-idl` feature.

</span><span class="kw">use </span><span class="kw">crate</span>::prelude::<span class="kw-2">*</span>;

<span class="comment">// The first 8 bytes of an instruction to create or modify the IDL account. This
// instruction is defined outside the main program's instruction enum, so that
// the enum variant tags can align with function source order.
//
// Sha256(anchor:idl)[..8];
</span><span class="kw">pub const </span>IDL_IX_TAG: u64 = <span class="number">0x0a69e9a778bcf440</span>;
<span class="kw">pub const </span>IDL_IX_TAG_LE: [u8; <span class="number">8</span>] = IDL_IX_TAG.to_le_bytes();

<span class="comment">// The Pubkey that is stored as the 'authority' on the IdlAccount when the authority
// is "erased".
</span><span class="kw">pub const </span>ERASED_AUTHORITY: Pubkey = Pubkey::new_from_array([<span class="number">0u8</span>; <span class="number">32</span>]);

<span class="attr">#[derive(AnchorSerialize, AnchorDeserialize)]
</span><span class="kw">pub enum </span>IdlInstruction {
    <span class="comment">// One time initializer for creating the program's idl account.
    </span>Create { data_len: u64 },
    <span class="comment">// Creates a new IDL account buffer. Can be called several times.
    </span>CreateBuffer,
    <span class="comment">// Appends the given data to the end of the idl account buffer.
    </span>Write { data: Vec&lt;u8&gt; },
    <span class="comment">// Sets a new data buffer for the IdlAccount.
    </span>SetBuffer,
    <span class="comment">// Sets a new authority on the IdlAccount.
    </span>SetAuthority { new_authority: Pubkey },
    Close,
    <span class="comment">// Increases account size for accounts that need over 10kb.
    </span>Resize { data_len: u64 },
}

<span class="comment">// The account holding a program's IDL. This is stored on chain so that clients
// can fetch it and generate a client with nothing but a program's ID.
//
// Note: we use the same account for the "write buffer", similar to the
//       bpf upgradeable loader's mechanism.
//
// TODO: IdlAccount exists here only because it's needed by the CLI, the IDL
// itself uses an IdlAccount defined inside the program itself, see program/idl.rs.
// Ideally it would be deleted and a better solution for sharing the type with CLI
// could be found.
</span><span class="attr">#[account(<span class="string">"internal"</span>)]
#[derive(Debug)]
</span><span class="kw">pub struct </span>IdlAccount {
    <span class="comment">// Address that can modify the IDL.
    </span><span class="kw">pub </span>authority: Pubkey,
    <span class="comment">// Length of compressed idl bytes.
    </span><span class="kw">pub </span>data_len: u32,
    <span class="comment">// Followed by compressed idl bytes.
</span>}

<span class="kw">impl </span>IdlAccount {
    <span class="kw">pub fn </span>address(program_id: <span class="kw-2">&amp;</span>Pubkey) -&gt; Pubkey {
        <span class="kw">let </span>program_signer = Pubkey::find_program_address(<span class="kw-2">&amp;</span>[], program_id).<span class="number">0</span>;
        Pubkey::create_with_seed(<span class="kw-2">&amp;</span>program_signer, IdlAccount::seed(), program_id)
            .expect(<span class="string">"Seed is always valid"</span>)
    }
    <span class="kw">pub fn </span>seed() -&gt; <span class="kw-2">&amp;</span><span class="lifetime">'static </span>str {
        <span class="string">"anchor:idl"
    </span>}
}

<span class="attr">#[cfg(feature = <span class="string">"idl-build"</span>)]
</span><span class="kw">pub use </span>anchor_lang_idl::{build::IdlBuild, <span class="kw-2">*</span>};
</code></pre></div></section></main></body></html>