<!DOCTYPE html><html lang="en"><head><meta charset="utf-8"><meta name="viewport" content="width=device-width, initial-scale=1.0"><meta name="generator" content="rustdoc"><meta name="description" content="Source of the Rust file `/Users/jeyakumar/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ark-ff-0.4.2/src/fields/field_hashers/mod.rs`."><title>mod.rs - source</title><script>if(window.location.protocol!=="file:")document.head.insertAdjacentHTML("beforeend","SourceSerif4-Regular-46f98efaafac5295.ttf.woff2,FiraSans-Regular-018c141bf0843ffd.woff2,FiraSans-Medium-8f9a781e4970d388.woff2,SourceCodePro-Regular-562dcc5011b6de7d.ttf.woff2,SourceCodePro-Semibold-d899c5a5c4aeb14a.ttf.woff2".split(",").map(f=>`<link rel="preload" as="font" type="font/woff2" crossorigin href="../../../../static.files/${f}">`).join(""))</script><link rel="stylesheet" href="../../../../static.files/normalize-76eba96aa4d2e634.css"><link rel="stylesheet" href="../../../../static.files/rustdoc-081576b923113409.css"><meta name="rustdoc-vars" data-root-path="../../../../" data-static-root-path="../../../../static.files/" data-current-crate="ark_ff" data-themes="" data-resource-suffix="" data-rustdoc-version="1.79.0 (129f3b996 2024-06-10)" data-channel="1.79.0" data-search-js="search-bf21c90c8c1d92b1.js" data-settings-js="settings-4313503d2e1961c2.js" ><script src="../../../../static.files/storage-e32f0c247825364d.js"></script><script defer src="../../../../static.files/src-script-e66d777a5a92e9b2.js"></script><script defer src="../../../../src-files.js"></script><script defer src="../../../../static.files/main-20a3ad099b048cf2.js"></script><noscript><link rel="stylesheet" href="../../../../static.files/noscript-09095024cf37855e.css"></noscript><link rel="alternate icon" type="image/png" href="../../../../static.files/favicon-32x32-422f7d1d52889060.png"><link rel="icon" type="image/svg+xml" href="../../../../static.files/favicon-2c020d218678b618.svg"></head><body class="rustdoc src"><!--[if lte IE 11]><div class="warning">This old browser is unsupported and will most likely display funky things.</div><![endif]--><nav class="sidebar"><div class="src-sidebar-title"><h2>Files</h2></div></nav><div class="sidebar-resizer"></div><main><nav class="sub"><form class="search-form"><span></span><div id="sidebar-button" tabindex="-1"><a href="../../../../ark_ff/all.html" title="show sidebar"></a></div><input class="search-input" name="search" aria-label="Run search in the documentation" autocomplete="off" spellcheck="false" placeholder="Type ‘S’ or ‘/’ to search, ‘?’ for more options…" type="search"><div id="help-button" tabindex="-1"><a href="../../../../help.html" title="help">?</a></div><div id="settings-menu" tabindex="-1"><a href="../../../../settings.html" title="settings">Settings</a></div></form></nav><section id="main-content" class="content"><div class="example-wrap"><div data-nosnippet><pre class="src-line-numbers"><a href="#1" id="1">1</a>
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
</pre></div><pre class="rust"><code><span class="kw">mod </span>expander;

<span class="kw">use crate</span>::{Field, PrimeField};

<span class="kw">use </span>ark_std::vec::Vec;
<span class="kw">use </span>digest::DynDigest;
<span class="kw">use </span>expander::Expander;

<span class="kw">use </span><span class="self">self</span>::expander::ExpanderXmd;

<span class="doccomment">/// Trait for hashing messages to field elements.
</span><span class="kw">pub trait </span>HashToField&lt;F: Field&gt;: Sized {
    <span class="doccomment">/// Initialises a new hash-to-field helper struct.
    ///
    /// # Arguments
    ///
    /// * `domain` - bytes that get concatenated with the `msg` during hashing, in order to separate potentially interfering instantiations of the hasher.
    </span><span class="kw">fn </span>new(domain: <span class="kw-2">&amp;</span>[u8]) -&gt; <span class="self">Self</span>;

    <span class="doccomment">/// Hash an arbitrary `msg` to #`count` elements from field `F`.
    </span><span class="kw">fn </span>hash_to_field(<span class="kw-2">&amp;</span><span class="self">self</span>, msg: <span class="kw-2">&amp;</span>[u8], count: usize) -&gt; Vec&lt;F&gt;;
}

<span class="doccomment">/// This field hasher constructs a Hash-To-Field based on a fixed-output hash function,
/// like SHA2, SHA3 or Blake2.
/// The implementation aims to follow the specification in [Hashing to Elliptic Curves (draft)](https://tools.ietf.org/pdf/draft-irtf-cfrg-hash-to-curve-13.pdf).
///
/// # Examples
///
/// ```
/// use ark_ff::fields::field_hashers::{DefaultFieldHasher, HashToField};
/// use ark_test_curves::bls12_381::Fq;
/// use sha2::Sha256;
///
/// let hasher = &lt;DefaultFieldHasher&lt;Sha256&gt; as HashToField&lt;Fq&gt;&gt;::new(&amp;[1, 2, 3]);
/// let field_elements: Vec&lt;Fq&gt; = hasher.hash_to_field(b"Hello, World!", 2);
///
/// assert_eq!(field_elements.len(), 2);
/// ```
</span><span class="kw">pub struct </span>DefaultFieldHasher&lt;H: Default + DynDigest + Clone, <span class="kw">const </span>SEC_PARAM: usize = <span class="number">128</span>&gt; {
    expander: ExpanderXmd&lt;H&gt;,
    len_per_base_elem: usize,
}

<span class="kw">impl</span>&lt;F: Field, H: Default + DynDigest + Clone, <span class="kw">const </span>SEC_PARAM: usize&gt; HashToField&lt;F&gt;
    <span class="kw">for </span>DefaultFieldHasher&lt;H, SEC_PARAM&gt;
{
    <span class="kw">fn </span>new(dst: <span class="kw-2">&amp;</span>[u8]) -&gt; <span class="self">Self </span>{
        <span class="comment">// The final output of `hash_to_field` will be an array of field
        // elements from F::BaseField, each of size `len_per_elem`.
        </span><span class="kw">let </span>len_per_base_elem = get_len_per_elem::&lt;F, SEC_PARAM&gt;();

        <span class="kw">let </span>expander = ExpanderXmd {
            hasher: H::default(),
            dst: dst.to_vec(),
            block_size: len_per_base_elem,
        };

        DefaultFieldHasher {
            expander,
            len_per_base_elem,
        }
    }

    <span class="kw">fn </span>hash_to_field(<span class="kw-2">&amp;</span><span class="self">self</span>, message: <span class="kw-2">&amp;</span>[u8], count: usize) -&gt; Vec&lt;F&gt; {
        <span class="kw">let </span>m = F::extension_degree() <span class="kw">as </span>usize;

        <span class="comment">// The user imposes a `count` of elements of F_p^m to output per input msg,
        // each field element comprising `m` BasePrimeField elements.
        </span><span class="kw">let </span>len_in_bytes = count * m * <span class="self">self</span>.len_per_base_elem;
        <span class="kw">let </span>uniform_bytes = <span class="self">self</span>.expander.expand(message, len_in_bytes);

        <span class="kw">let </span><span class="kw-2">mut </span>output = Vec::with_capacity(count);
        <span class="kw">let </span><span class="kw-2">mut </span>base_prime_field_elems = Vec::with_capacity(m);
        <span class="kw">for </span>i <span class="kw">in </span><span class="number">0</span>..count {
            base_prime_field_elems.clear();
            <span class="kw">for </span>j <span class="kw">in </span><span class="number">0</span>..m {
                <span class="kw">let </span>elm_offset = <span class="self">self</span>.len_per_base_elem * (j + i * m);
                <span class="kw">let </span>val = F::BasePrimeField::from_be_bytes_mod_order(
                    <span class="kw-2">&amp;</span>uniform_bytes[elm_offset..][..<span class="self">self</span>.len_per_base_elem],
                );
                base_prime_field_elems.push(val);
            }
            <span class="kw">let </span>f = F::from_base_prime_field_elems(<span class="kw-2">&amp;</span>base_prime_field_elems).unwrap();
            output.push(f);
        }

        output
    }
}

<span class="doccomment">/// This function computes the length in bytes that a hash function should output
/// for hashing an element of type `Field`.
/// See section 5.1 and 5.3 of the
/// [IETF hash standardization draft](https://datatracker.ietf.org/doc/draft-irtf-cfrg-hash-to-curve/14/)
</span><span class="kw">fn </span>get_len_per_elem&lt;F: Field, <span class="kw">const </span>SEC_PARAM: usize&gt;() -&gt; usize {
    <span class="comment">// ceil(log(p))
    </span><span class="kw">let </span>base_field_size_in_bits = F::BasePrimeField::MODULUS_BIT_SIZE <span class="kw">as </span>usize;
    <span class="comment">// ceil(log(p)) + security_parameter
    </span><span class="kw">let </span>base_field_size_with_security_padding_in_bits = base_field_size_in_bits + SEC_PARAM;
    <span class="comment">// ceil( (ceil(log(p)) + security_parameter) / 8)
    </span><span class="kw">let </span>bytes_per_base_field_elem =
        ((base_field_size_with_security_padding_in_bits + <span class="number">7</span>) / <span class="number">8</span>) <span class="kw">as </span>u64;
    bytes_per_base_field_elem <span class="kw">as </span>usize
}
</code></pre></div></section></main></body></html>