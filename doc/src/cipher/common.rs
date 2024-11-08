<!DOCTYPE html><html lang="en"><head><meta charset="utf-8"><meta name="viewport" content="width=device-width, initial-scale=1.0"><meta name="generator" content="rustdoc"><meta name="description" content="Source of the Rust file `/Users/jeyakumar/.cargo/registry/src/index.crates.io-6f17d22bba15001f/cipher-0.3.0/src/common.rs`."><title>common.rs - source</title><script>if(window.location.protocol!=="file:")document.head.insertAdjacentHTML("beforeend","SourceSerif4-Regular-46f98efaafac5295.ttf.woff2,FiraSans-Regular-018c141bf0843ffd.woff2,FiraSans-Medium-8f9a781e4970d388.woff2,SourceCodePro-Regular-562dcc5011b6de7d.ttf.woff2,SourceCodePro-Semibold-d899c5a5c4aeb14a.ttf.woff2".split(",").map(f=>`<link rel="preload" as="font" type="font/woff2" crossorigin href="../../static.files/${f}">`).join(""))</script><link rel="stylesheet" href="../../static.files/normalize-76eba96aa4d2e634.css"><link rel="stylesheet" href="../../static.files/rustdoc-081576b923113409.css"><meta name="rustdoc-vars" data-root-path="../../" data-static-root-path="../../static.files/" data-current-crate="cipher" data-themes="" data-resource-suffix="" data-rustdoc-version="1.79.0 (129f3b996 2024-06-10)" data-channel="1.79.0" data-search-js="search-bf21c90c8c1d92b1.js" data-settings-js="settings-4313503d2e1961c2.js" ><script src="../../static.files/storage-e32f0c247825364d.js"></script><script defer src="../../static.files/src-script-e66d777a5a92e9b2.js"></script><script defer src="../../src-files.js"></script><script defer src="../../static.files/main-20a3ad099b048cf2.js"></script><noscript><link rel="stylesheet" href="../../static.files/noscript-09095024cf37855e.css"></noscript><link rel="icon" href="https://raw.githubusercontent.com/RustCrypto/meta/master/logo.svg"></head><body class="rustdoc src"><!--[if lte IE 11]><div class="warning">This old browser is unsupported and will most likely display funky things.</div><![endif]--><nav class="sidebar"><div class="src-sidebar-title"><h2>Files</h2></div></nav><div class="sidebar-resizer"></div><main><nav class="sub"><form class="search-form"><span></span><div id="sidebar-button" tabindex="-1"><a href="../../cipher/all.html" title="show sidebar"></a></div><input class="search-input" name="search" aria-label="Run search in the documentation" autocomplete="off" spellcheck="false" placeholder="Type ‘S’ or ‘/’ to search, ‘?’ for more options…" type="search"><div id="help-button" tabindex="-1"><a href="../../help.html" title="help">?</a></div><div id="settings-menu" tabindex="-1"><a href="../../settings.html" title="settings">Settings</a></div></form></nav><section id="main-content" class="content"><div class="example-wrap"><div data-nosnippet><pre class="src-line-numbers"><a href="#1" id="1">1</a>
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
</pre></div><pre class="rust"><code><span class="kw">use crate</span>::{errors::InvalidLength, BlockCipher, NewBlockCipher};
<span class="kw">use </span>generic_array::{typenum::Unsigned, ArrayLength, GenericArray};

<span class="doccomment">/// Key for an algorithm that implements [`NewCipher`].
</span><span class="kw">pub type </span>CipherKey&lt;C&gt; = GenericArray&lt;u8, &lt;C <span class="kw">as </span>NewCipher&gt;::KeySize&gt;;

<span class="doccomment">/// Nonce for an algorithm that implements [`NewCipher`].
</span><span class="kw">pub type </span>Nonce&lt;C&gt; = GenericArray&lt;u8, &lt;C <span class="kw">as </span>NewCipher&gt;::NonceSize&gt;;

<span class="doccomment">/// Cipher creation trait.
///
/// It can be used for creation of block modes, synchronous and asynchronous stream ciphers.
</span><span class="kw">pub trait </span>NewCipher: Sized {
    <span class="doccomment">/// Key size in bytes
    </span><span class="kw">type </span>KeySize: ArrayLength&lt;u8&gt;;

    <span class="doccomment">/// Nonce size in bytes
    </span><span class="kw">type </span>NonceSize: ArrayLength&lt;u8&gt;;

    <span class="doccomment">/// Create new stream cipher instance from key and nonce arrays.
    </span><span class="kw">fn </span>new(key: <span class="kw-2">&amp;</span>CipherKey&lt;<span class="self">Self</span>&gt;, nonce: <span class="kw-2">&amp;</span>Nonce&lt;<span class="self">Self</span>&gt;) -&gt; <span class="self">Self</span>;

    <span class="doccomment">/// Create new stream cipher instance from variable length key and nonce
    /// given as byte slices.
    </span><span class="attr">#[inline]
    </span><span class="kw">fn </span>new_from_slices(key: <span class="kw-2">&amp;</span>[u8], nonce: <span class="kw-2">&amp;</span>[u8]) -&gt; <span class="prelude-ty">Result</span>&lt;<span class="self">Self</span>, InvalidLength&gt; {
        <span class="kw">let </span>kl = <span class="self">Self</span>::KeySize::to_usize();
        <span class="kw">let </span>nl = <span class="self">Self</span>::NonceSize::to_usize();
        <span class="kw">if </span>key.len() != kl || nonce.len() != nl {
            <span class="prelude-val">Err</span>(InvalidLength)
        } <span class="kw">else </span>{
            <span class="kw">let </span>key = GenericArray::from_slice(key);
            <span class="kw">let </span>nonce = GenericArray::from_slice(nonce);
            <span class="prelude-val">Ok</span>(<span class="self">Self</span>::new(key, nonce))
        }
    }
}

<span class="doccomment">/// Trait for types which can be initialized from a block cipher and nonce.
</span><span class="kw">pub trait </span>FromBlockCipher {
    <span class="doccomment">/// Block cipher
    </span><span class="kw">type </span>BlockCipher: BlockCipher;
    <span class="doccomment">/// Nonce size in bytes
    </span><span class="kw">type </span>NonceSize: ArrayLength&lt;u8&gt;;

    <span class="doccomment">/// Instantiate a stream cipher from a block cipher
    </span><span class="kw">fn </span>from_block_cipher(
        cipher: <span class="self">Self</span>::BlockCipher,
        nonce: <span class="kw-2">&amp;</span>GenericArray&lt;u8, <span class="self">Self</span>::NonceSize&gt;,
    ) -&gt; <span class="self">Self</span>;
}

<span class="kw">impl</span>&lt;C&gt; NewCipher <span class="kw">for </span>C
<span class="kw">where
    </span>C: FromBlockCipher,
    C::BlockCipher: NewBlockCipher,
{
    <span class="kw">type </span>KeySize = &lt;&lt;<span class="self">Self </span><span class="kw">as </span>FromBlockCipher&gt;::BlockCipher <span class="kw">as </span>NewBlockCipher&gt;::KeySize;
    <span class="kw">type </span>NonceSize = &lt;<span class="self">Self </span><span class="kw">as </span>FromBlockCipher&gt;::NonceSize;

    <span class="kw">fn </span>new(key: <span class="kw-2">&amp;</span>CipherKey&lt;<span class="self">Self</span>&gt;, nonce: <span class="kw-2">&amp;</span>Nonce&lt;<span class="self">Self</span>&gt;) -&gt; C {
        C::from_block_cipher(
            &lt;&lt;<span class="self">Self </span><span class="kw">as </span>FromBlockCipher&gt;::BlockCipher <span class="kw">as </span>NewBlockCipher&gt;::new(key),
            nonce,
        )
    }

    <span class="kw">fn </span>new_from_slices(key: <span class="kw-2">&amp;</span>[u8], nonce: <span class="kw-2">&amp;</span>[u8]) -&gt; <span class="prelude-ty">Result</span>&lt;<span class="self">Self</span>, InvalidLength&gt; {
        <span class="kw">if </span>nonce.len() != <span class="self">Self</span>::NonceSize::USIZE {
            <span class="prelude-val">Err</span>(InvalidLength)
        } <span class="kw">else </span>{
            C::BlockCipher::new_from_slice(key)
                .map_err(|<span class="kw">_</span>| InvalidLength)
                .map(|cipher| {
                    <span class="kw">let </span>nonce = GenericArray::from_slice(nonce);
                    <span class="self">Self</span>::from_block_cipher(cipher, nonce)
                })
        }
    }
}
</code></pre></div></section></main></body></html>