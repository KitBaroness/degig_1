<!DOCTYPE html><html lang="en"><head><meta charset="utf-8"><meta name="viewport" content="width=device-width, initial-scale=1.0"><meta name="generator" content="rustdoc"><meta name="description" content="Source of the Rust file `/Users/jeyakumar/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ark-ff-0.4.2/src/fields/fft_friendly.rs`."><title>fft_friendly.rs - source</title><script>if(window.location.protocol!=="file:")document.head.insertAdjacentHTML("beforeend","SourceSerif4-Regular-46f98efaafac5295.ttf.woff2,FiraSans-Regular-018c141bf0843ffd.woff2,FiraSans-Medium-8f9a781e4970d388.woff2,SourceCodePro-Regular-562dcc5011b6de7d.ttf.woff2,SourceCodePro-Semibold-d899c5a5c4aeb14a.ttf.woff2".split(",").map(f=>`<link rel="preload" as="font" type="font/woff2" crossorigin href="../../../static.files/${f}">`).join(""))</script><link rel="stylesheet" href="../../../static.files/normalize-76eba96aa4d2e634.css"><link rel="stylesheet" href="../../../static.files/rustdoc-081576b923113409.css"><meta name="rustdoc-vars" data-root-path="../../../" data-static-root-path="../../../static.files/" data-current-crate="ark_ff" data-themes="" data-resource-suffix="" data-rustdoc-version="1.79.0 (129f3b996 2024-06-10)" data-channel="1.79.0" data-search-js="search-bf21c90c8c1d92b1.js" data-settings-js="settings-4313503d2e1961c2.js" ><script src="../../../static.files/storage-e32f0c247825364d.js"></script><script defer src="../../../static.files/src-script-e66d777a5a92e9b2.js"></script><script defer src="../../../src-files.js"></script><script defer src="../../../static.files/main-20a3ad099b048cf2.js"></script><noscript><link rel="stylesheet" href="../../../static.files/noscript-09095024cf37855e.css"></noscript><link rel="alternate icon" type="image/png" href="../../../static.files/favicon-32x32-422f7d1d52889060.png"><link rel="icon" type="image/svg+xml" href="../../../static.files/favicon-2c020d218678b618.svg"></head><body class="rustdoc src"><!--[if lte IE 11]><div class="warning">This old browser is unsupported and will most likely display funky things.</div><![endif]--><nav class="sidebar"><div class="src-sidebar-title"><h2>Files</h2></div></nav><div class="sidebar-resizer"></div><main><nav class="sub"><form class="search-form"><span></span><div id="sidebar-button" tabindex="-1"><a href="../../../ark_ff/all.html" title="show sidebar"></a></div><input class="search-input" name="search" aria-label="Run search in the documentation" autocomplete="off" spellcheck="false" placeholder="Type ‘S’ or ‘/’ to search, ‘?’ for more options…" type="search"><div id="help-button" tabindex="-1"><a href="../../../help.html" title="help">?</a></div><div id="settings-menu" tabindex="-1"><a href="../../../settings.html" title="settings">Settings</a></div></form></nav><section id="main-content" class="content"><div class="example-wrap"><div data-nosnippet><pre class="src-line-numbers"><a href="#1" id="1">1</a>
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
</pre></div><pre class="rust"><code><span class="doccomment">/// The interface for fields that are able to be used in FFTs.
</span><span class="kw">pub trait </span>FftField: <span class="kw">crate</span>::Field {
    <span class="doccomment">/// The generator of the multiplicative group of the field
    </span><span class="kw">const </span>GENERATOR: <span class="self">Self</span>;

    <span class="doccomment">/// Let `N` be the size of the multiplicative group defined by the field.
    /// Then `TWO_ADICITY` is the two-adicity of `N`, i.e. the integer `s`
    /// such that `N = 2^s * t` for some odd integer `t`.
    </span><span class="kw">const </span>TWO_ADICITY: u32;

    <span class="doccomment">/// 2^s root of unity computed by GENERATOR^t
    </span><span class="kw">const </span>TWO_ADIC_ROOT_OF_UNITY: <span class="self">Self</span>;

    <span class="doccomment">/// An integer `b` such that there exists a multiplicative subgroup
    /// of size `b^k` for some integer `k`.
    </span><span class="kw">const </span>SMALL_SUBGROUP_BASE: <span class="prelude-ty">Option</span>&lt;u32&gt; = <span class="prelude-val">None</span>;

    <span class="doccomment">/// The integer `k` such that there exists a multiplicative subgroup
    /// of size `Self::SMALL_SUBGROUP_BASE^k`.
    </span><span class="kw">const </span>SMALL_SUBGROUP_BASE_ADICITY: <span class="prelude-ty">Option</span>&lt;u32&gt; = <span class="prelude-val">None</span>;

    <span class="doccomment">/// GENERATOR^((MODULUS-1) / (2^s *
    /// SMALL_SUBGROUP_BASE^SMALL_SUBGROUP_BASE_ADICITY)) Used for mixed-radix
    /// FFT.
    </span><span class="kw">const </span>LARGE_SUBGROUP_ROOT_OF_UNITY: <span class="prelude-ty">Option</span>&lt;<span class="self">Self</span>&gt; = <span class="prelude-val">None</span>;

    <span class="doccomment">/// Returns the root of unity of order n, if one exists.
    /// If no small multiplicative subgroup is defined, this is the 2-adic root
    /// of unity of order n (for n a power of 2).
    /// If a small multiplicative subgroup is defined, this is the root of unity
    /// of order n for the larger subgroup generated by
    /// `FftConfig::LARGE_SUBGROUP_ROOT_OF_UNITY`
    /// (for n = 2^i * FftConfig::SMALL_SUBGROUP_BASE^j for some i, j).
    </span><span class="kw">fn </span>get_root_of_unity(n: u64) -&gt; <span class="prelude-ty">Option</span>&lt;<span class="self">Self</span>&gt; {
        <span class="kw">let </span><span class="kw-2">mut </span>omega: <span class="self">Self</span>;
        <span class="kw">if let </span><span class="prelude-val">Some</span>(large_subgroup_root_of_unity) = <span class="self">Self</span>::LARGE_SUBGROUP_ROOT_OF_UNITY {
            <span class="kw">let </span>q = <span class="self">Self</span>::SMALL_SUBGROUP_BASE.expect(
                <span class="string">"LARGE_SUBGROUP_ROOT_OF_UNITY should only be set in conjunction with SMALL_SUBGROUP_BASE"</span>,
            ) <span class="kw">as </span>u64;
            <span class="kw">let </span>small_subgroup_base_adicity = <span class="self">Self</span>::SMALL_SUBGROUP_BASE_ADICITY.expect(
                <span class="string">"LARGE_SUBGROUP_ROOT_OF_UNITY should only be set in conjunction with SMALL_SUBGROUP_BASE_ADICITY"</span>,
            );

            <span class="kw">let </span>q_adicity = <span class="kw">crate</span>::utils::k_adicity(q, n);
            <span class="kw">let </span>q_part = q.checked_pow(q_adicity)<span class="question-mark">?</span>;

            <span class="kw">let </span>two_adicity = <span class="kw">crate</span>::utils::k_adicity(<span class="number">2</span>, n);
            <span class="kw">let </span>two_part = <span class="number">2u64</span>.checked_pow(two_adicity)<span class="question-mark">?</span>;

            <span class="kw">if </span>n != two_part * q_part
                || (two_adicity &gt; <span class="self">Self</span>::TWO_ADICITY)
                || (q_adicity &gt; small_subgroup_base_adicity)
            {
                <span class="kw">return </span><span class="prelude-val">None</span>;
            }

            omega = large_subgroup_root_of_unity;
            <span class="kw">for _ in </span>q_adicity..small_subgroup_base_adicity {
                omega = omega.pow([q <span class="kw">as </span>u64]);
            }

            <span class="kw">for _ in </span>two_adicity..<span class="self">Self</span>::TWO_ADICITY {
                omega.square_in_place();
            }
        } <span class="kw">else </span>{
            <span class="comment">// Compute the next power of 2.
            </span><span class="kw">let </span>size = n.next_power_of_two() <span class="kw">as </span>u64;
            <span class="kw">let </span>log_size_of_group = ark_std::log2(usize::try_from(size).expect(<span class="string">"too large"</span>));

            <span class="kw">if </span>n != size || log_size_of_group &gt; <span class="self">Self</span>::TWO_ADICITY {
                <span class="kw">return </span><span class="prelude-val">None</span>;
            }

            <span class="comment">// Compute the generator for the multiplicative subgroup.
            // It should be 2^(log_size_of_group) root of unity.
            </span>omega = <span class="self">Self</span>::TWO_ADIC_ROOT_OF_UNITY;
            <span class="kw">for _ in </span>log_size_of_group..<span class="self">Self</span>::TWO_ADICITY {
                omega.square_in_place();
            }
        }
        <span class="prelude-val">Some</span>(omega)
    }
}
</code></pre></div></section></main></body></html>