<!DOCTYPE html><html lang="en"><head><meta charset="utf-8"><meta name="viewport" content="width=device-width, initial-scale=1.0"><meta name="generator" content="rustdoc"><meta name="description" content="Source of the Rust file `/Users/jeyakumar/.cargo/registry/src/index.crates.io-6f17d22bba15001f/http-0.2.12/src/lib.rs`."><title>lib.rs - source</title><script>if(window.location.protocol!=="file:")document.head.insertAdjacentHTML("beforeend","SourceSerif4-Regular-46f98efaafac5295.ttf.woff2,FiraSans-Regular-018c141bf0843ffd.woff2,FiraSans-Medium-8f9a781e4970d388.woff2,SourceCodePro-Regular-562dcc5011b6de7d.ttf.woff2,SourceCodePro-Semibold-d899c5a5c4aeb14a.ttf.woff2".split(",").map(f=>`<link rel="preload" as="font" type="font/woff2" crossorigin href="../../static.files/${f}">`).join(""))</script><link rel="stylesheet" href="../../static.files/normalize-76eba96aa4d2e634.css"><link rel="stylesheet" href="../../static.files/rustdoc-081576b923113409.css"><meta name="rustdoc-vars" data-root-path="../../" data-static-root-path="../../static.files/" data-current-crate="http" data-themes="" data-resource-suffix="" data-rustdoc-version="1.79.0 (129f3b996 2024-06-10)" data-channel="1.79.0" data-search-js="search-bf21c90c8c1d92b1.js" data-settings-js="settings-4313503d2e1961c2.js" ><script src="../../static.files/storage-e32f0c247825364d.js"></script><script defer src="../../static.files/src-script-e66d777a5a92e9b2.js"></script><script defer src="../../src-files.js"></script><script defer src="../../static.files/main-20a3ad099b048cf2.js"></script><noscript><link rel="stylesheet" href="../../static.files/noscript-09095024cf37855e.css"></noscript><link rel="alternate icon" type="image/png" href="../../static.files/favicon-32x32-422f7d1d52889060.png"><link rel="icon" type="image/svg+xml" href="../../static.files/favicon-2c020d218678b618.svg"></head><body class="rustdoc src"><!--[if lte IE 11]><div class="warning">This old browser is unsupported and will most likely display funky things.</div><![endif]--><nav class="sidebar"><div class="src-sidebar-title"><h2>Files</h2></div></nav><div class="sidebar-resizer"></div><main><nav class="sub"><form class="search-form"><span></span><div id="sidebar-button" tabindex="-1"><a href="../../http/all.html" title="show sidebar"></a></div><input class="search-input" name="search" aria-label="Run search in the documentation" autocomplete="off" spellcheck="false" placeholder="Type ‘S’ or ‘/’ to search, ‘?’ for more options…" type="search"><div id="help-button" tabindex="-1"><a href="../../help.html" title="help">?</a></div><div id="settings-menu" tabindex="-1"><a href="../../settings.html" title="settings">Settings</a></div></form></nav><section id="main-content" class="content"><div class="example-wrap"><div data-nosnippet><pre class="src-line-numbers"><a href="#1" id="1">1</a>
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
</pre></div><pre class="rust"><code><span class="attr">#![doc(html_root_url = <span class="string">"https://docs.rs/http/0.2.11"</span>)]

</span><span class="doccomment">//! A general purpose library of common HTTP types
//!
//! This crate is a general purpose library for common types found when working
//! with the HTTP protocol. You'll find `Request` and `Response` types for
//! working as either a client or a server as well as all of their components.
//! Notably you'll find `Uri` for what a `Request` is requesting, a `Method`
//! for how it's being requested, a `StatusCode` for what sort of response came
//! back, a `Version` for how this was communicated, and
//! `HeaderName`/`HeaderValue` definitions to get grouped in a `HeaderMap` to
//! work with request/response headers.
//!
//! You will notably *not* find an implementation of sending requests or
//! spinning up a server in this crate. It's intended that this crate is the
//! "standard library" for HTTP clients and servers without dictating any
//! particular implementation. Note that this crate is still early on in its
//! lifecycle so the support libraries that integrate with the `http` crate are
//! a work in progress! Stay tuned and we'll be sure to highlight crates here
//! in the future.
//!
//! ## Requests and Responses
//!
//! Perhaps the main two types in this crate are the `Request` and `Response`
//! types. A `Request` could either be constructed to get sent off as a client
//! or it can also be received to generate a `Response` for a server. Similarly
//! as a client a `Response` is what you get after sending a `Request`, whereas
//! on a server you'll be manufacturing a `Response` to send back to the client.
//!
//! Each type has a number of accessors for the component fields. For as a
//! server you might want to inspect a requests URI to dispatch it:
//!
//! ```
//! use http::{Request, Response};
//!
//! fn response(req: Request&lt;()&gt;) -&gt; http::Result&lt;Response&lt;()&gt;&gt; {
//!     match req.uri().path() {
//!         "/" =&gt; index(req),
//!         "/foo" =&gt; foo(req),
//!         "/bar" =&gt; bar(req),
//!         _ =&gt; not_found(req),
//!     }
//! }
//! # fn index(_req: Request&lt;()&gt;) -&gt; http::Result&lt;Response&lt;()&gt;&gt; { panic!() }
//! # fn foo(_req: Request&lt;()&gt;) -&gt; http::Result&lt;Response&lt;()&gt;&gt; { panic!() }
//! # fn bar(_req: Request&lt;()&gt;) -&gt; http::Result&lt;Response&lt;()&gt;&gt; { panic!() }
//! # fn not_found(_req: Request&lt;()&gt;) -&gt; http::Result&lt;Response&lt;()&gt;&gt; { panic!() }
//! ```
//!
//! On a `Request` you'll also find accessors like `method` to return a
//! `Method` and `headers` to inspect the various headers. A `Response`
//! has similar methods for headers, the status code, etc.
//!
//! In addition to getters, request/response types also have mutable accessors
//! to edit the request/response:
//!
//! ```
//! use http::{HeaderValue, Response, StatusCode};
//! use http::header::CONTENT_TYPE;
//!
//! fn add_server_headers&lt;T&gt;(response: &amp;mut Response&lt;T&gt;) {
//!     response.headers_mut()
//!         .insert(CONTENT_TYPE, HeaderValue::from_static("text/html"));
//!     *response.status_mut() = StatusCode::OK;
//! }
//! ```
//!
//! And finally, one of the most important aspects of requests/responses, the
//! body! The `Request` and `Response` types in this crate are *generic* in
//! what their body is. This allows downstream libraries to use different
//! representations such as `Request&lt;Vec&lt;u8&gt;&gt;`, `Response&lt;impl Read&gt;`,
//! `Request&lt;impl Stream&lt;Item = Vec&lt;u8&gt;, Error = _&gt;&gt;`, or even
//! `Response&lt;MyCustomType&gt;` where the custom type was deserialized from JSON.
//!
//! The body representation is intentionally flexible to give downstream
//! libraries maximal flexibility in implementing the body as appropriate.
//!
//! ## HTTP Headers
//!
//! Another major piece of functionality in this library is HTTP header
//! interpretation and generation. The `HeaderName` type serves as a way to
//! define header *names*, or what's to the left of the colon. A `HeaderValue`
//! conversely is the header *value*, or what's to the right of a colon.
//!
//! For example, if you have an HTTP request that looks like:
//!
//! ```http
//! GET /foo HTTP/1.1
//! Accept: text/html
//! ```
//!
//! Then `"Accept"` is a `HeaderName` while `"text/html"` is a `HeaderValue`.
//! Each of these is a dedicated type to allow for a number of interesting
//! optimizations and to also encode the static guarantees of each type. For
//! example a `HeaderName` is always a valid `&amp;str`, but a `HeaderValue` may
//! not be valid UTF-8.
//!
//! The most common header names are already defined for you as constant values
//! in the `header` module of this crate. For example:
//!
//! ```
//! use http::header::{self, HeaderName};
//!
//! let name: HeaderName = header::ACCEPT;
//! assert_eq!(name.as_str(), "accept");
//! ```
//!
//! You can, however, also parse header names from strings:
//!
//! ```
//! use http::header::{self, HeaderName};
//!
//! let name = "Accept".parse::&lt;HeaderName&gt;().unwrap();
//! assert_eq!(name, header::ACCEPT);
//! ```
//!
//! Header values can be created from string literals through the `from_static`
//! function:
//!
//! ```
//! use http::HeaderValue;
//!
//! let value = HeaderValue::from_static("text/html");
//! assert_eq!(value.as_bytes(), b"text/html");
//! ```
//!
//! And header values can also be parsed like names:
//!
//! ```
//! use http::HeaderValue;
//!
//! let value = "text/html";
//! let value = value.parse::&lt;HeaderValue&gt;().unwrap();
//! ```
//!
//! Most HTTP requests and responses tend to come with more than one header, so
//! it's not too useful to just work with names and values only! This crate also
//! provides a `HeaderMap` type which is a specialized hash map for keys as
//! `HeaderName` and generic values. This type, like header names, is optimized
//! for common usage but should continue to scale with your needs over time.
//!
//! # URIs
//!
//! Each HTTP `Request` has an associated URI with it. This may just be a path
//! like `/index.html` but it could also be an absolute URL such as
//! `https://www.rust-lang.org/index.html`. A `URI` has a number of accessors to
//! interpret it:
//!
//! ```
//! use http::Uri;
//! use http::uri::Scheme;
//!
//! let uri = "https://www.rust-lang.org/index.html".parse::&lt;Uri&gt;().unwrap();
//!
//! assert_eq!(uri.scheme(), Some(&amp;Scheme::HTTPS));
//! assert_eq!(uri.host(), Some("www.rust-lang.org"));
//! assert_eq!(uri.path(), "/index.html");
//! assert_eq!(uri.query(), None);
//! ```

</span><span class="attr">#![deny(missing_docs, missing_debug_implementations)]

#[cfg(test)]
#[macro_use]
</span><span class="kw">extern crate </span>doc_comment;

<span class="attr">#[cfg(test)]
</span><span class="macro">doctest!</span>(<span class="string">"../README.md"</span>);

<span class="attr">#[macro_use]
</span><span class="kw">mod </span>convert;

<span class="kw">pub mod </span>header;
<span class="kw">pub mod </span>method;
<span class="kw">pub mod </span>request;
<span class="kw">pub mod </span>response;
<span class="kw">pub mod </span>status;
<span class="kw">pub mod </span>uri;
<span class="kw">pub mod </span>version;

<span class="kw">mod </span>byte_str;
<span class="kw">mod </span>error;
<span class="kw">mod </span>extensions;

<span class="kw">pub use </span><span class="kw">crate</span>::error::{Error, <span class="prelude-ty">Result</span>};
<span class="kw">pub use </span><span class="kw">crate</span>::extensions::Extensions;
<span class="attr">#[doc(no_inline)]
</span><span class="kw">pub use </span><span class="kw">crate</span>::header::{HeaderMap, HeaderName, HeaderValue};
<span class="kw">pub use </span><span class="kw">crate</span>::method::Method;
<span class="kw">pub use </span><span class="kw">crate</span>::request::Request;
<span class="kw">pub use </span><span class="kw">crate</span>::response::Response;
<span class="kw">pub use </span><span class="kw">crate</span>::status::StatusCode;
<span class="kw">pub use </span><span class="kw">crate</span>::uri::Uri;
<span class="kw">pub use </span><span class="kw">crate</span>::version::Version;

<span class="kw">fn </span>_assert_types() {
    <span class="kw">fn </span>assert_send&lt;T: Send&gt;() {}
    <span class="kw">fn </span>assert_sync&lt;T: Sync&gt;() {}

    assert_send::&lt;Request&lt;()&gt;&gt;();
    assert_send::&lt;Response&lt;()&gt;&gt;();

    assert_sync::&lt;Request&lt;()&gt;&gt;();
    assert_sync::&lt;Response&lt;()&gt;&gt;();
}

<span class="kw">mod </span>sealed {
    <span class="doccomment">/// Private trait to this crate to prevent traits from being implemented in
    /// downstream crates.
    </span><span class="kw">pub trait </span>Sealed {}
}
</code></pre></div></section></main></body></html>