<!DOCTYPE html><html lang="en"><head><meta charset="utf-8"><meta name="viewport" content="width=device-width, initial-scale=1.0"><meta name="generator" content="rustdoc"><meta name="description" content="Source of the Rust file `/Users/jeyakumar/.cargo/registry/src/index.crates.io-6f17d22bba15001f/derivative-2.2.0/src/hash.rs`."><title>hash.rs - source</title><script>if(window.location.protocol!=="file:")document.head.insertAdjacentHTML("beforeend","SourceSerif4-Regular-46f98efaafac5295.ttf.woff2,FiraSans-Regular-018c141bf0843ffd.woff2,FiraSans-Medium-8f9a781e4970d388.woff2,SourceCodePro-Regular-562dcc5011b6de7d.ttf.woff2,SourceCodePro-Semibold-d899c5a5c4aeb14a.ttf.woff2".split(",").map(f=>`<link rel="preload" as="font" type="font/woff2" crossorigin href="../../static.files/${f}">`).join(""))</script><link rel="stylesheet" href="../../static.files/normalize-76eba96aa4d2e634.css"><link rel="stylesheet" href="../../static.files/rustdoc-081576b923113409.css"><meta name="rustdoc-vars" data-root-path="../../" data-static-root-path="../../static.files/" data-current-crate="derivative" data-themes="" data-resource-suffix="" data-rustdoc-version="1.79.0 (129f3b996 2024-06-10)" data-channel="1.79.0" data-search-js="search-bf21c90c8c1d92b1.js" data-settings-js="settings-4313503d2e1961c2.js" ><script src="../../static.files/storage-e32f0c247825364d.js"></script><script defer src="../../static.files/src-script-e66d777a5a92e9b2.js"></script><script defer src="../../src-files.js"></script><script defer src="../../static.files/main-20a3ad099b048cf2.js"></script><noscript><link rel="stylesheet" href="../../static.files/noscript-09095024cf37855e.css"></noscript><link rel="alternate icon" type="image/png" href="../../static.files/favicon-32x32-422f7d1d52889060.png"><link rel="icon" type="image/svg+xml" href="../../static.files/favicon-2c020d218678b618.svg"></head><body class="rustdoc src"><!--[if lte IE 11]><div class="warning">This old browser is unsupported and will most likely display funky things.</div><![endif]--><nav class="sidebar"><div class="src-sidebar-title"><h2>Files</h2></div></nav><div class="sidebar-resizer"></div><main><nav class="sub"><form class="search-form"><span></span><div id="sidebar-button" tabindex="-1"><a href="../../derivative/all.html" title="show sidebar"></a></div><input class="search-input" name="search" aria-label="Run search in the documentation" autocomplete="off" spellcheck="false" placeholder="Type ‘S’ or ‘/’ to search, ‘?’ for more options…" type="search"><div id="help-button" tabindex="-1"><a href="../../help.html" title="help">?</a></div><div id="settings-menu" tabindex="-1"><a href="../../settings.html" title="settings">Settings</a></div></form></nav><section id="main-content" class="content"><div class="example-wrap"><div data-nosnippet><pre class="src-line-numbers"><a href="#1" id="1">1</a>
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
</pre></div><pre class="rust"><code><span class="kw">use </span>proc_macro2;

<span class="kw">use </span>ast;
<span class="kw">use </span>attr;
<span class="kw">use </span>matcher;
<span class="kw">use </span>paths;
<span class="kw">use </span>syn;
<span class="kw">use </span>utils;

<span class="kw">pub fn </span>derive(input: <span class="kw-2">&amp;</span>ast::Input) -&gt; proc_macro2::TokenStream {
    <span class="kw">let </span>hasher_trait_path = hasher_trait_path();
    <span class="kw">let </span>hash_trait_path = hash_trait_path();

    <span class="kw">let </span>discriminant = <span class="kw">if let </span>ast::Body::Enum(<span class="kw">_</span>) = input.body {
        <span class="kw">let </span>discriminant = paths::discriminant_path();
        <span class="prelude-val">Some</span>(<span class="macro">quote!</span>(
            #hash_trait_path::hash(<span class="kw-2">&amp;</span>#discriminant(<span class="self">self</span>), __state);
        ))
    } <span class="kw">else </span>{
        <span class="prelude-val">None
    </span>};

    <span class="kw">let </span>body = matcher::Matcher::new(matcher::BindingStyle::Ref, input.attrs.is_packed).build_arms(
        input,
        <span class="string">"__arg"</span>,
        |<span class="kw">_</span>, <span class="kw">_</span>, <span class="kw">_</span>, <span class="kw">_</span>, <span class="kw">_</span>, bis| {
            <span class="kw">let </span>field_prints = bis.iter().filter_map(|bi| {
                <span class="kw">if </span>bi.field.attrs.ignore_hash() {
                    <span class="kw">return </span><span class="prelude-val">None</span>;
                }

                <span class="kw">let </span>arg = <span class="kw-2">&amp;</span>bi.expr;

                <span class="kw">if let </span><span class="prelude-val">Some</span>(hash_with) = bi.field.attrs.hash_with() {
                    <span class="prelude-val">Some</span>(<span class="macro">quote!</span> {
                        #hash_with(<span class="kw-2">&amp;</span>#arg, __state);
                    })
                } <span class="kw">else </span>{
                    <span class="prelude-val">Some</span>(<span class="macro">quote!</span> {
                        #hash_trait_path::hash(<span class="kw-2">&amp;</span>#arg, __state);
                    })
                }
            });

            <span class="macro">quote!</span> {
                #(#field_prints)*
            }
        },
    );

    <span class="kw">let </span>name = <span class="kw-2">&amp;</span>input.ident;
    <span class="kw">let </span>generics = utils::build_impl_generics(
        input,
        <span class="kw-2">&amp;</span>hash_trait_path,
        needs_hash_bound,
        |field| field.hash_bound(),
        |input| input.hash_bound(),
    );
    <span class="kw">let </span>(impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    <span class="kw">let </span>hasher_ty_parameter = utils::hygienic_type_parameter(input, <span class="string">"__H"</span>);
    <span class="macro">quote!</span> {
        <span class="attr">#[allow(unused_qualifications)]
        </span><span class="kw">impl </span>#impl_generics #hash_trait_path <span class="kw">for </span>#name #ty_generics #where_clause {
            <span class="kw">fn </span>hash&lt;#hasher_ty_parameter&gt;(<span class="kw-2">&amp;</span><span class="self">self</span>, __state: <span class="kw-2">&amp;mut </span>#hasher_ty_parameter)
                <span class="kw">where </span>#hasher_ty_parameter: #hasher_trait_path
            {
                #discriminant
                <span class="kw">match </span><span class="kw-2">*</span><span class="self">self </span>{
                    #body
                }
            }
        }
    }
}

<span class="kw">fn </span>needs_hash_bound(attrs: <span class="kw-2">&amp;</span>attr::Field) -&gt; bool {
    !attrs.ignore_hash() &amp;&amp; attrs.hash_bound().is_none()
}

<span class="doccomment">/// Return the path of the `Hash` trait, that is `::std::hash::Hash`.
</span><span class="kw">fn </span>hash_trait_path() -&gt; syn::Path {
    <span class="kw">if </span><span class="macro">cfg!</span>(feature = <span class="string">"use_core"</span>) {
        <span class="macro">parse_quote!</span>(::core::hash::Hash)
    } <span class="kw">else </span>{
        <span class="macro">parse_quote!</span>(::std::hash::Hash)
    }
}

<span class="doccomment">/// Return the path of the `Hasher` trait, that is `::std::hash::Hasher`.
</span><span class="kw">fn </span>hasher_trait_path() -&gt; syn::Path {
    <span class="kw">if </span><span class="macro">cfg!</span>(feature = <span class="string">"use_core"</span>) {
        <span class="macro">parse_quote!</span>(::core::hash::Hasher)
    } <span class="kw">else </span>{
        <span class="macro">parse_quote!</span>(::std::hash::Hasher)
    }
}
</code></pre></div></section></main></body></html>