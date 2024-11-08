<!DOCTYPE html><html lang="en"><head><meta charset="utf-8"><meta name="viewport" content="width=device-width, initial-scale=1.0"><meta name="generator" content="rustdoc"><meta name="description" content="Source of the Rust file `/Users/jeyakumar/.cargo/registry/src/index.crates.io-6f17d22bba15001f/tokio-1.37.0/src/runtime/scheduler/mod.rs`."><title>mod.rs - source</title><script>if(window.location.protocol!=="file:")document.head.insertAdjacentHTML("beforeend","SourceSerif4-Regular-46f98efaafac5295.ttf.woff2,FiraSans-Regular-018c141bf0843ffd.woff2,FiraSans-Medium-8f9a781e4970d388.woff2,SourceCodePro-Regular-562dcc5011b6de7d.ttf.woff2,SourceCodePro-Semibold-d899c5a5c4aeb14a.ttf.woff2".split(",").map(f=>`<link rel="preload" as="font" type="font/woff2" crossorigin href="../../../../static.files/${f}">`).join(""))</script><link rel="stylesheet" href="../../../../static.files/normalize-76eba96aa4d2e634.css"><link rel="stylesheet" href="../../../../static.files/rustdoc-081576b923113409.css"><meta name="rustdoc-vars" data-root-path="../../../../" data-static-root-path="../../../../static.files/" data-current-crate="tokio" data-themes="" data-resource-suffix="" data-rustdoc-version="1.79.0 (129f3b996 2024-06-10)" data-channel="1.79.0" data-search-js="search-bf21c90c8c1d92b1.js" data-settings-js="settings-4313503d2e1961c2.js" ><script src="../../../../static.files/storage-e32f0c247825364d.js"></script><script defer src="../../../../static.files/src-script-e66d777a5a92e9b2.js"></script><script defer src="../../../../src-files.js"></script><script defer src="../../../../static.files/main-20a3ad099b048cf2.js"></script><noscript><link rel="stylesheet" href="../../../../static.files/noscript-09095024cf37855e.css"></noscript><link rel="alternate icon" type="image/png" href="../../../../static.files/favicon-32x32-422f7d1d52889060.png"><link rel="icon" type="image/svg+xml" href="../../../../static.files/favicon-2c020d218678b618.svg"></head><body class="rustdoc src"><!--[if lte IE 11]><div class="warning">This old browser is unsupported and will most likely display funky things.</div><![endif]--><nav class="sidebar"><div class="src-sidebar-title"><h2>Files</h2></div></nav><div class="sidebar-resizer"></div><main><nav class="sub"><form class="search-form"><span></span><div id="sidebar-button" tabindex="-1"><a href="../../../../tokio/all.html" title="show sidebar"></a></div><input class="search-input" name="search" aria-label="Run search in the documentation" autocomplete="off" spellcheck="false" placeholder="Type ‘S’ or ‘/’ to search, ‘?’ for more options…" type="search"><div id="help-button" tabindex="-1"><a href="../../../../help.html" title="help">?</a></div><div id="settings-menu" tabindex="-1"><a href="../../../../settings.html" title="settings">Settings</a></div></form></nav><section id="main-content" class="content"><div class="example-wrap"><div data-nosnippet><pre class="src-line-numbers"><a href="#1" id="1">1</a>
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
</pre></div><pre class="rust"><code><span class="macro">cfg_rt!</span> {
    <span class="kw">pub</span>(<span class="kw">crate</span>) <span class="kw">mod </span>current_thread;
    <span class="kw">pub</span>(<span class="kw">crate</span>) <span class="kw">use </span>current_thread::CurrentThread;

    <span class="kw">mod </span>defer;
    <span class="kw">use </span>defer::Defer;

    <span class="kw">pub</span>(<span class="kw">crate</span>) <span class="kw">mod </span>inject;
    <span class="kw">pub</span>(<span class="kw">crate</span>) <span class="kw">use </span>inject::Inject;
}

<span class="macro">cfg_rt_multi_thread!</span> {
    <span class="kw">mod </span>block_in_place;
    <span class="kw">pub</span>(<span class="kw">crate</span>) <span class="kw">use </span>block_in_place::block_in_place;

    <span class="kw">mod </span>lock;
    <span class="kw">use </span>lock::Lock;

    <span class="kw">pub</span>(<span class="kw">crate</span>) <span class="kw">mod </span>multi_thread;
    <span class="kw">pub</span>(<span class="kw">crate</span>) <span class="kw">use </span>multi_thread::MultiThread;

    <span class="macro">cfg_unstable!</span> {
        <span class="kw">pub</span>(<span class="kw">crate</span>) <span class="kw">mod </span>multi_thread_alt;
        <span class="kw">pub</span>(<span class="kw">crate</span>) <span class="kw">use </span>multi_thread_alt::MultiThread <span class="kw">as </span>MultiThreadAlt;
    }
}

<span class="kw">use </span><span class="kw">crate</span>::runtime::driver;

<span class="attr">#[derive(Debug, Clone)]
</span><span class="kw">pub</span>(<span class="kw">crate</span>) <span class="kw">enum </span>Handle {
    <span class="attr">#[cfg(feature = <span class="string">"rt"</span>)]
    </span>CurrentThread(Arc&lt;current_thread::Handle&gt;),

    <span class="attr">#[cfg(all(feature = <span class="string">"rt-multi-thread"</span>, not(target_os = <span class="string">"wasi"</span>)))]
    </span>MultiThread(Arc&lt;multi_thread::Handle&gt;),

    <span class="attr">#[cfg(all(tokio_unstable, feature = <span class="string">"rt-multi-thread"</span>, not(target_os = <span class="string">"wasi"</span>)))]
    </span>MultiThreadAlt(Arc&lt;multi_thread_alt::Handle&gt;),

    <span class="comment">// TODO: This is to avoid triggering "dead code" warnings many other places
    // in the codebase. Remove this during a later cleanup
    </span><span class="attr">#[cfg(not(feature = <span class="string">"rt"</span>))]
    #[allow(dead_code)]
    </span>Disabled,
}

<span class="attr">#[cfg(feature = <span class="string">"rt"</span>)]
</span><span class="kw">pub</span>(<span class="kw">super</span>) <span class="kw">enum </span>Context {
    CurrentThread(current_thread::Context),

    <span class="attr">#[cfg(all(feature = <span class="string">"rt-multi-thread"</span>, not(target_os = <span class="string">"wasi"</span>)))]
    </span>MultiThread(multi_thread::Context),

    <span class="attr">#[cfg(all(tokio_unstable, feature = <span class="string">"rt-multi-thread"</span>, not(target_os = <span class="string">"wasi"</span>)))]
    </span>MultiThreadAlt(multi_thread_alt::Context),
}

<span class="kw">impl </span>Handle {
    <span class="attr">#[cfg_attr(not(feature = <span class="string">"full"</span>), allow(dead_code))]
    </span><span class="kw">pub</span>(<span class="kw">crate</span>) <span class="kw">fn </span>driver(<span class="kw-2">&amp;</span><span class="self">self</span>) -&gt; <span class="kw-2">&amp;</span>driver::Handle {
        <span class="kw">match </span><span class="kw-2">*</span><span class="self">self </span>{
            <span class="attr">#[cfg(feature = <span class="string">"rt"</span>)]
            </span>Handle::CurrentThread(<span class="kw-2">ref </span>h) =&gt; <span class="kw-2">&amp;</span>h.driver,

            <span class="attr">#[cfg(all(feature = <span class="string">"rt-multi-thread"</span>, not(target_os = <span class="string">"wasi"</span>)))]
            </span>Handle::MultiThread(<span class="kw-2">ref </span>h) =&gt; <span class="kw-2">&amp;</span>h.driver,

            <span class="attr">#[cfg(all(tokio_unstable, feature = <span class="string">"rt-multi-thread"</span>, not(target_os = <span class="string">"wasi"</span>)))]
            </span>Handle::MultiThreadAlt(<span class="kw-2">ref </span>h) =&gt; <span class="kw-2">&amp;</span>h.driver,

            <span class="attr">#[cfg(not(feature = <span class="string">"rt"</span>))]
            </span>Handle::Disabled =&gt; <span class="macro">unreachable!</span>(),
        }
    }
}

<span class="macro">cfg_rt!</span> {
    <span class="kw">use </span><span class="kw">crate</span>::future::Future;
    <span class="kw">use </span><span class="kw">crate</span>::loom::sync::Arc;
    <span class="kw">use </span><span class="kw">crate</span>::runtime::{blocking, task::Id};
    <span class="kw">use </span><span class="kw">crate</span>::runtime::context;
    <span class="kw">use </span><span class="kw">crate</span>::task::JoinHandle;
    <span class="kw">use </span><span class="kw">crate</span>::util::RngSeedGenerator;
    <span class="kw">use </span>std::task::Waker;

    <span class="macro">macro_rules!</span> match_flavor {
        (<span class="macro-nonterminal">$self</span>:expr, <span class="macro-nonterminal">$ty</span>:ident(<span class="macro-nonterminal">$h</span>:ident) =&gt; <span class="macro-nonterminal">$e</span>:expr) =&gt; {
            <span class="kw">match </span><span class="macro-nonterminal">$self </span>{
                <span class="macro-nonterminal">$ty::CurrentThread</span>(<span class="macro-nonterminal">$h</span>) =&gt; <span class="macro-nonterminal">$e</span>,

                <span class="attr">#[cfg(all(feature = <span class="string">"rt-multi-thread"</span>, not(target_os = <span class="string">"wasi"</span>)))]
                </span><span class="macro-nonterminal">$ty::MultiThread</span>(<span class="macro-nonterminal">$h</span>) =&gt; <span class="macro-nonterminal">$e</span>,

                <span class="attr">#[cfg(all(tokio_unstable, feature = <span class="string">"rt-multi-thread"</span>, not(target_os = <span class="string">"wasi"</span>)))]
                </span><span class="macro-nonterminal">$ty::MultiThreadAlt</span>(<span class="macro-nonterminal">$h</span>) =&gt; <span class="macro-nonterminal">$e</span>,
            }
        }
    }

    <span class="kw">impl </span>Handle {
        <span class="attr">#[track_caller]
        </span><span class="kw">pub</span>(<span class="kw">crate</span>) <span class="kw">fn </span>current() -&gt; Handle {
            <span class="kw">match </span>context::with_current(Clone::clone) {
                <span class="prelude-val">Ok</span>(handle) =&gt; handle,
                <span class="prelude-val">Err</span>(e) =&gt; <span class="macro">panic!</span>(<span class="string">"{}"</span>, e),
            }
        }

        <span class="kw">pub</span>(<span class="kw">crate</span>) <span class="kw">fn </span>blocking_spawner(<span class="kw-2">&amp;</span><span class="self">self</span>) -&gt; <span class="kw-2">&amp;</span>blocking::Spawner {
            <span class="macro">match_flavor!</span>(<span class="self">self</span>, Handle(h) =&gt; <span class="kw-2">&amp;</span>h.blocking_spawner)
        }

        <span class="kw">pub</span>(<span class="kw">crate</span>) <span class="kw">fn </span>spawn&lt;F&gt;(<span class="kw-2">&amp;</span><span class="self">self</span>, future: F, id: Id) -&gt; JoinHandle&lt;F::Output&gt;
        <span class="kw">where
            </span>F: Future + Send + <span class="lifetime">'static</span>,
            F::Output: Send + <span class="lifetime">'static</span>,
        {
            <span class="kw">match </span><span class="self">self </span>{
                Handle::CurrentThread(h) =&gt; current_thread::Handle::spawn(h, future, id),

                <span class="attr">#[cfg(all(feature = <span class="string">"rt-multi-thread"</span>, not(target_os = <span class="string">"wasi"</span>)))]
                </span>Handle::MultiThread(h) =&gt; multi_thread::Handle::spawn(h, future, id),

                <span class="attr">#[cfg(all(tokio_unstable, feature = <span class="string">"rt-multi-thread"</span>, not(target_os = <span class="string">"wasi"</span>)))]
                </span>Handle::MultiThreadAlt(h) =&gt; multi_thread_alt::Handle::spawn(h, future, id),
            }
        }

        <span class="kw">pub</span>(<span class="kw">crate</span>) <span class="kw">fn </span>shutdown(<span class="kw-2">&amp;</span><span class="self">self</span>) {
            <span class="kw">match </span><span class="kw-2">*</span><span class="self">self </span>{
                Handle::CurrentThread(<span class="kw">_</span>) =&gt; {},

                <span class="attr">#[cfg(all(feature = <span class="string">"rt-multi-thread"</span>, not(target_os = <span class="string">"wasi"</span>)))]
                </span>Handle::MultiThread(<span class="kw-2">ref </span>h) =&gt; h.shutdown(),

                <span class="attr">#[cfg(all(tokio_unstable, feature = <span class="string">"rt-multi-thread"</span>, not(target_os = <span class="string">"wasi"</span>)))]
                </span>Handle::MultiThreadAlt(<span class="kw-2">ref </span>h) =&gt; h.shutdown(),
            }
        }

        <span class="kw">pub</span>(<span class="kw">crate</span>) <span class="kw">fn </span>seed_generator(<span class="kw-2">&amp;</span><span class="self">self</span>) -&gt; <span class="kw-2">&amp;</span>RngSeedGenerator {
            <span class="macro">match_flavor!</span>(<span class="self">self</span>, Handle(h) =&gt; <span class="kw-2">&amp;</span>h.seed_generator)
        }

        <span class="kw">pub</span>(<span class="kw">crate</span>) <span class="kw">fn </span>as_current_thread(<span class="kw-2">&amp;</span><span class="self">self</span>) -&gt; <span class="kw-2">&amp;</span>Arc&lt;current_thread::Handle&gt; {
            <span class="kw">match </span><span class="self">self </span>{
                Handle::CurrentThread(handle) =&gt; handle,
                <span class="attr">#[cfg(all(feature = <span class="string">"rt-multi-thread"</span>, not(target_os = <span class="string">"wasi"</span>)))]
                </span><span class="kw">_ </span>=&gt; <span class="macro">panic!</span>(<span class="string">"not a CurrentThread handle"</span>),
            }
        }

        <span class="macro">cfg_rt_multi_thread!</span> {
            <span class="macro">cfg_unstable!</span> {
                <span class="kw">pub</span>(<span class="kw">crate</span>) <span class="kw">fn </span>expect_multi_thread_alt(<span class="kw-2">&amp;</span><span class="self">self</span>) -&gt; <span class="kw-2">&amp;</span>Arc&lt;multi_thread_alt::Handle&gt; {
                    <span class="kw">match </span><span class="self">self </span>{
                        Handle::MultiThreadAlt(handle) =&gt; handle,
                        <span class="kw">_ </span>=&gt; <span class="macro">panic!</span>(<span class="string">"not a `MultiThreadAlt` handle"</span>),
                    }
                }
            }
        }
    }

    <span class="macro">cfg_metrics!</span> {
        <span class="kw">use </span><span class="kw">crate</span>::runtime::{SchedulerMetrics, WorkerMetrics};

        <span class="kw">impl </span>Handle {
            <span class="kw">pub</span>(<span class="kw">crate</span>) <span class="kw">fn </span>num_workers(<span class="kw-2">&amp;</span><span class="self">self</span>) -&gt; usize {
                <span class="kw">match </span><span class="self">self </span>{
                    Handle::CurrentThread(<span class="kw">_</span>) =&gt; <span class="number">1</span>,
                    <span class="attr">#[cfg(all(feature = <span class="string">"rt-multi-thread"</span>, not(target_os = <span class="string">"wasi"</span>)))]
                    </span>Handle::MultiThread(handle) =&gt; handle.num_workers(),
                    <span class="attr">#[cfg(all(tokio_unstable, feature = <span class="string">"rt-multi-thread"</span>, not(target_os = <span class="string">"wasi"</span>)))]
                    </span>Handle::MultiThreadAlt(handle) =&gt; handle.num_workers(),
                }
            }

            <span class="kw">pub</span>(<span class="kw">crate</span>) <span class="kw">fn </span>num_blocking_threads(<span class="kw-2">&amp;</span><span class="self">self</span>) -&gt; usize {
                <span class="macro">match_flavor!</span>(<span class="self">self</span>, Handle(handle) =&gt; handle.num_blocking_threads())
            }

            <span class="kw">pub</span>(<span class="kw">crate</span>) <span class="kw">fn </span>num_idle_blocking_threads(<span class="kw-2">&amp;</span><span class="self">self</span>) -&gt; usize {
                <span class="macro">match_flavor!</span>(<span class="self">self</span>, Handle(handle) =&gt; handle.num_idle_blocking_threads())
            }

            <span class="kw">pub</span>(<span class="kw">crate</span>) <span class="kw">fn </span>active_tasks_count(<span class="kw-2">&amp;</span><span class="self">self</span>) -&gt; usize {
                <span class="macro">match_flavor!</span>(<span class="self">self</span>, Handle(handle) =&gt; handle.active_tasks_count())
            }

            <span class="kw">pub</span>(<span class="kw">crate</span>) <span class="kw">fn </span>scheduler_metrics(<span class="kw-2">&amp;</span><span class="self">self</span>) -&gt; <span class="kw-2">&amp;</span>SchedulerMetrics {
                <span class="macro">match_flavor!</span>(<span class="self">self</span>, Handle(handle) =&gt; handle.scheduler_metrics())
            }

            <span class="kw">pub</span>(<span class="kw">crate</span>) <span class="kw">fn </span>worker_metrics(<span class="kw-2">&amp;</span><span class="self">self</span>, worker: usize) -&gt; <span class="kw-2">&amp;</span>WorkerMetrics {
                <span class="macro">match_flavor!</span>(<span class="self">self</span>, Handle(handle) =&gt; handle.worker_metrics(worker))
            }

            <span class="kw">pub</span>(<span class="kw">crate</span>) <span class="kw">fn </span>injection_queue_depth(<span class="kw-2">&amp;</span><span class="self">self</span>) -&gt; usize {
                <span class="macro">match_flavor!</span>(<span class="self">self</span>, Handle(handle) =&gt; handle.injection_queue_depth())
            }

            <span class="kw">pub</span>(<span class="kw">crate</span>) <span class="kw">fn </span>worker_local_queue_depth(<span class="kw-2">&amp;</span><span class="self">self</span>, worker: usize) -&gt; usize {
                <span class="macro">match_flavor!</span>(<span class="self">self</span>, Handle(handle) =&gt; handle.worker_local_queue_depth(worker))
            }

            <span class="kw">pub</span>(<span class="kw">crate</span>) <span class="kw">fn </span>blocking_queue_depth(<span class="kw-2">&amp;</span><span class="self">self</span>) -&gt; usize {
                <span class="macro">match_flavor!</span>(<span class="self">self</span>, Handle(handle) =&gt; handle.blocking_queue_depth())
            }
        }
    }

    <span class="kw">impl </span>Context {
        <span class="attr">#[track_caller]
        </span><span class="kw">pub</span>(<span class="kw">crate</span>) <span class="kw">fn </span>expect_current_thread(<span class="kw-2">&amp;</span><span class="self">self</span>) -&gt; <span class="kw-2">&amp;</span>current_thread::Context {
            <span class="kw">match </span><span class="self">self </span>{
                Context::CurrentThread(context) =&gt; context,
                <span class="attr">#[cfg(all(feature = <span class="string">"rt-multi-thread"</span>, not(target_os = <span class="string">"wasi"</span>)))]
                </span><span class="kw">_ </span>=&gt; <span class="macro">panic!</span>(<span class="string">"expected `CurrentThread::Context`"</span>)
            }
        }

        <span class="kw">pub</span>(<span class="kw">crate</span>) <span class="kw">fn </span>defer(<span class="kw-2">&amp;</span><span class="self">self</span>, waker: <span class="kw-2">&amp;</span>Waker) {
            <span class="macro">match_flavor!</span>(<span class="self">self</span>, Context(context) =&gt; context.defer(waker));
        }

        <span class="macro">cfg_rt_multi_thread!</span> {
            <span class="attr">#[track_caller]
            </span><span class="kw">pub</span>(<span class="kw">crate</span>) <span class="kw">fn </span>expect_multi_thread(<span class="kw-2">&amp;</span><span class="self">self</span>) -&gt; <span class="kw-2">&amp;</span>multi_thread::Context {
                <span class="kw">match </span><span class="self">self </span>{
                    Context::MultiThread(context) =&gt; context,
                    <span class="kw">_ </span>=&gt; <span class="macro">panic!</span>(<span class="string">"expected `MultiThread::Context`"</span>)
                }
            }

            <span class="macro">cfg_unstable!</span> {
                <span class="attr">#[track_caller]
                </span><span class="kw">pub</span>(<span class="kw">crate</span>) <span class="kw">fn </span>expect_multi_thread_alt(<span class="kw-2">&amp;</span><span class="self">self</span>) -&gt; <span class="kw-2">&amp;</span>multi_thread_alt::Context {
                    <span class="kw">match </span><span class="self">self </span>{
                        Context::MultiThreadAlt(context) =&gt; context,
                        <span class="kw">_ </span>=&gt; <span class="macro">panic!</span>(<span class="string">"expected `MultiThreadAlt::Context`"</span>)
                    }
                }
            }
        }
    }
}

<span class="macro">cfg_not_rt!</span> {
    <span class="attr">#[cfg(any(
        feature = <span class="string">"net"</span>,
        all(unix, feature = <span class="string">"process"</span>),
        all(unix, feature = <span class="string">"signal"</span>),
        feature = <span class="string">"time"</span>,
    ))]
    </span><span class="kw">impl </span>Handle {
        <span class="attr">#[track_caller]
        </span><span class="kw">pub</span>(<span class="kw">crate</span>) <span class="kw">fn </span>current() -&gt; Handle {
            <span class="macro">panic!</span>(<span class="string">"{}"</span>, <span class="kw">crate</span>::util::error::CONTEXT_MISSING_ERROR)
        }
    }
}
</code></pre></div></section></main></body></html>