<!DOCTYPE html><html lang="en"><head><meta charset="utf-8"><meta name="viewport" content="width=device-width, initial-scale=1.0"><meta name="generator" content="rustdoc"><meta name="description" content="Source of the Rust file `/Users/jeyakumar/.cargo/registry/src/index.crates.io-6f17d22bba15001f/chrono-0.4.38/src/format/scan.rs`."><title>scan.rs - source</title><script>if(window.location.protocol!=="file:")document.head.insertAdjacentHTML("beforeend","SourceSerif4-Regular-46f98efaafac5295.ttf.woff2,FiraSans-Regular-018c141bf0843ffd.woff2,FiraSans-Medium-8f9a781e4970d388.woff2,SourceCodePro-Regular-562dcc5011b6de7d.ttf.woff2,SourceCodePro-Semibold-d899c5a5c4aeb14a.ttf.woff2".split(",").map(f=>`<link rel="preload" as="font" type="font/woff2" crossorigin href="../../../static.files/${f}">`).join(""))</script><link rel="stylesheet" href="../../../static.files/normalize-76eba96aa4d2e634.css"><link rel="stylesheet" href="../../../static.files/rustdoc-081576b923113409.css"><meta name="rustdoc-vars" data-root-path="../../../" data-static-root-path="../../../static.files/" data-current-crate="chrono" data-themes="" data-resource-suffix="" data-rustdoc-version="1.79.0 (129f3b996 2024-06-10)" data-channel="1.79.0" data-search-js="search-bf21c90c8c1d92b1.js" data-settings-js="settings-4313503d2e1961c2.js" ><script src="../../../static.files/storage-e32f0c247825364d.js"></script><script defer src="../../../static.files/src-script-e66d777a5a92e9b2.js"></script><script defer src="../../../src-files.js"></script><script defer src="../../../static.files/main-20a3ad099b048cf2.js"></script><noscript><link rel="stylesheet" href="../../../static.files/noscript-09095024cf37855e.css"></noscript><link rel="alternate icon" type="image/png" href="../../../static.files/favicon-32x32-422f7d1d52889060.png"><link rel="icon" type="image/svg+xml" href="../../../static.files/favicon-2c020d218678b618.svg"></head><body class="rustdoc src"><!--[if lte IE 11]><div class="warning">This old browser is unsupported and will most likely display funky things.</div><![endif]--><nav class="sidebar"><div class="src-sidebar-title"><h2>Files</h2></div></nav><div class="sidebar-resizer"></div><main><nav class="sub"><form class="search-form"><span></span><div id="sidebar-button" tabindex="-1"><a href="../../../chrono/all.html" title="show sidebar"></a></div><input class="search-input" name="search" aria-label="Run search in the documentation" autocomplete="off" spellcheck="false" placeholder="Type ‘S’ or ‘/’ to search, ‘?’ for more options…" type="search"><div id="help-button" tabindex="-1"><a href="../../../help.html" title="help">?</a></div><div id="settings-menu" tabindex="-1"><a href="../../../settings.html" title="settings">Settings</a></div></form></nav><section id="main-content" class="content"><div class="example-wrap"><div data-nosnippet><pre class="src-line-numbers"><a href="#1" id="1">1</a>
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
<a href="#329" id="329">329</a>
<a href="#330" id="330">330</a>
<a href="#331" id="331">331</a>
<a href="#332" id="332">332</a>
<a href="#333" id="333">333</a>
<a href="#334" id="334">334</a>
<a href="#335" id="335">335</a>
<a href="#336" id="336">336</a>
<a href="#337" id="337">337</a>
<a href="#338" id="338">338</a>
<a href="#339" id="339">339</a>
<a href="#340" id="340">340</a>
<a href="#341" id="341">341</a>
<a href="#342" id="342">342</a>
<a href="#343" id="343">343</a>
<a href="#344" id="344">344</a>
<a href="#345" id="345">345</a>
<a href="#346" id="346">346</a>
<a href="#347" id="347">347</a>
<a href="#348" id="348">348</a>
<a href="#349" id="349">349</a>
<a href="#350" id="350">350</a>
<a href="#351" id="351">351</a>
<a href="#352" id="352">352</a>
<a href="#353" id="353">353</a>
<a href="#354" id="354">354</a>
<a href="#355" id="355">355</a>
<a href="#356" id="356">356</a>
<a href="#357" id="357">357</a>
<a href="#358" id="358">358</a>
<a href="#359" id="359">359</a>
<a href="#360" id="360">360</a>
<a href="#361" id="361">361</a>
<a href="#362" id="362">362</a>
<a href="#363" id="363">363</a>
<a href="#364" id="364">364</a>
<a href="#365" id="365">365</a>
<a href="#366" id="366">366</a>
<a href="#367" id="367">367</a>
<a href="#368" id="368">368</a>
<a href="#369" id="369">369</a>
<a href="#370" id="370">370</a>
<a href="#371" id="371">371</a>
<a href="#372" id="372">372</a>
<a href="#373" id="373">373</a>
<a href="#374" id="374">374</a>
<a href="#375" id="375">375</a>
<a href="#376" id="376">376</a>
<a href="#377" id="377">377</a>
<a href="#378" id="378">378</a>
<a href="#379" id="379">379</a>
<a href="#380" id="380">380</a>
<a href="#381" id="381">381</a>
<a href="#382" id="382">382</a>
<a href="#383" id="383">383</a>
<a href="#384" id="384">384</a>
<a href="#385" id="385">385</a>
<a href="#386" id="386">386</a>
<a href="#387" id="387">387</a>
<a href="#388" id="388">388</a>
<a href="#389" id="389">389</a>
<a href="#390" id="390">390</a>
<a href="#391" id="391">391</a>
<a href="#392" id="392">392</a>
<a href="#393" id="393">393</a>
<a href="#394" id="394">394</a>
<a href="#395" id="395">395</a>
<a href="#396" id="396">396</a>
<a href="#397" id="397">397</a>
<a href="#398" id="398">398</a>
<a href="#399" id="399">399</a>
<a href="#400" id="400">400</a>
<a href="#401" id="401">401</a>
<a href="#402" id="402">402</a>
<a href="#403" id="403">403</a>
<a href="#404" id="404">404</a>
<a href="#405" id="405">405</a>
<a href="#406" id="406">406</a>
<a href="#407" id="407">407</a>
<a href="#408" id="408">408</a>
<a href="#409" id="409">409</a>
<a href="#410" id="410">410</a>
<a href="#411" id="411">411</a>
<a href="#412" id="412">412</a>
<a href="#413" id="413">413</a>
<a href="#414" id="414">414</a>
<a href="#415" id="415">415</a>
<a href="#416" id="416">416</a>
<a href="#417" id="417">417</a>
<a href="#418" id="418">418</a>
<a href="#419" id="419">419</a>
<a href="#420" id="420">420</a>
<a href="#421" id="421">421</a>
<a href="#422" id="422">422</a>
<a href="#423" id="423">423</a>
<a href="#424" id="424">424</a>
<a href="#425" id="425">425</a>
<a href="#426" id="426">426</a>
<a href="#427" id="427">427</a>
<a href="#428" id="428">428</a>
<a href="#429" id="429">429</a>
<a href="#430" id="430">430</a>
<a href="#431" id="431">431</a>
<a href="#432" id="432">432</a>
<a href="#433" id="433">433</a>
<a href="#434" id="434">434</a>
<a href="#435" id="435">435</a>
<a href="#436" id="436">436</a>
</pre></div><pre class="rust"><code><span class="comment">// This is a part of Chrono.
// See README.md and LICENSE.txt for details.

</span><span class="doccomment">/*!
 * Various scanning routines for the parser.
 */

</span><span class="kw">use super</span>::{ParseResult, INVALID, OUT_OF_RANGE, TOO_SHORT};
<span class="kw">use </span><span class="kw">crate</span>::Weekday;

<span class="doccomment">/// Tries to parse the non-negative number from `min` to `max` digits.
///
/// The absence of digits at all is an unconditional error.
/// More than `max` digits are consumed up to the first `max` digits.
/// Any number that does not fit in `i64` is an error.
</span><span class="attr">#[inline]
</span><span class="kw">pub</span>(<span class="kw">super</span>) <span class="kw">fn </span>number(s: <span class="kw-2">&amp;</span>str, min: usize, max: usize) -&gt; ParseResult&lt;(<span class="kw-2">&amp;</span>str, i64)&gt; {
    <span class="macro">assert!</span>(min &lt;= max);

    <span class="comment">// We are only interested in ascii numbers, so we can work with the `str` as bytes. We stop on
    // the first non-numeric byte, which may be another ascii character or beginning of multi-byte
    // UTF-8 character.
    </span><span class="kw">let </span>bytes = s.as_bytes();
    <span class="kw">if </span>bytes.len() &lt; min {
        <span class="kw">return </span><span class="prelude-val">Err</span>(TOO_SHORT);
    }

    <span class="kw">let </span><span class="kw-2">mut </span>n = <span class="number">0i64</span>;
    <span class="kw">for </span>(i, c) <span class="kw">in </span>bytes.iter().take(max).cloned().enumerate() {
        <span class="comment">// cloned() = copied()
        </span><span class="kw">if </span>!c.is_ascii_digit() {
            <span class="kw">if </span>i &lt; min {
                <span class="kw">return </span><span class="prelude-val">Err</span>(INVALID);
            } <span class="kw">else </span>{
                <span class="kw">return </span><span class="prelude-val">Ok</span>((<span class="kw-2">&amp;</span>s[i..], n));
            }
        }

        n = <span class="kw">match </span>n.checked_mul(<span class="number">10</span>).and_then(|n| n.checked_add((c - <span class="string">b'0'</span>) <span class="kw">as </span>i64)) {
            <span class="prelude-val">Some</span>(n) =&gt; n,
            <span class="prelude-val">None </span>=&gt; <span class="kw">return </span><span class="prelude-val">Err</span>(OUT_OF_RANGE),
        };
    }

    <span class="prelude-val">Ok</span>((<span class="kw-2">&amp;</span>s[core::cmp::min(max, bytes.len())..], n))
}

<span class="doccomment">/// Tries to consume at least one digits as a fractional second.
/// Returns the number of whole nanoseconds (0--999,999,999).
</span><span class="kw">pub</span>(<span class="kw">super</span>) <span class="kw">fn </span>nanosecond(s: <span class="kw-2">&amp;</span>str) -&gt; ParseResult&lt;(<span class="kw-2">&amp;</span>str, i64)&gt; {
    <span class="comment">// record the number of digits consumed for later scaling.
    </span><span class="kw">let </span>origlen = s.len();
    <span class="kw">let </span>(s, v) = number(s, <span class="number">1</span>, <span class="number">9</span>)<span class="question-mark">?</span>;
    <span class="kw">let </span>consumed = origlen - s.len();

    <span class="comment">// scale the number accordingly.
    </span><span class="kw">static </span>SCALE: [i64; <span class="number">10</span>] =
        [<span class="number">0</span>, <span class="number">100_000_000</span>, <span class="number">10_000_000</span>, <span class="number">1_000_000</span>, <span class="number">100_000</span>, <span class="number">10_000</span>, <span class="number">1_000</span>, <span class="number">100</span>, <span class="number">10</span>, <span class="number">1</span>];
    <span class="kw">let </span>v = v.checked_mul(SCALE[consumed]).ok_or(OUT_OF_RANGE)<span class="question-mark">?</span>;

    <span class="comment">// if there are more than 9 digits, skip next digits.
    </span><span class="kw">let </span>s = s.trim_start_matches(|c: char| c.is_ascii_digit());

    <span class="prelude-val">Ok</span>((s, v))
}

<span class="doccomment">/// Tries to consume a fixed number of digits as a fractional second.
/// Returns the number of whole nanoseconds (0--999,999,999).
</span><span class="kw">pub</span>(<span class="kw">super</span>) <span class="kw">fn </span>nanosecond_fixed(s: <span class="kw-2">&amp;</span>str, digits: usize) -&gt; ParseResult&lt;(<span class="kw-2">&amp;</span>str, i64)&gt; {
    <span class="comment">// record the number of digits consumed for later scaling.
    </span><span class="kw">let </span>(s, v) = number(s, digits, digits)<span class="question-mark">?</span>;

    <span class="comment">// scale the number accordingly.
    </span><span class="kw">static </span>SCALE: [i64; <span class="number">10</span>] =
        [<span class="number">0</span>, <span class="number">100_000_000</span>, <span class="number">10_000_000</span>, <span class="number">1_000_000</span>, <span class="number">100_000</span>, <span class="number">10_000</span>, <span class="number">1_000</span>, <span class="number">100</span>, <span class="number">10</span>, <span class="number">1</span>];
    <span class="kw">let </span>v = v.checked_mul(SCALE[digits]).ok_or(OUT_OF_RANGE)<span class="question-mark">?</span>;

    <span class="prelude-val">Ok</span>((s, v))
}

<span class="doccomment">/// Tries to parse the month index (0 through 11) with the first three ASCII letters.
</span><span class="kw">pub</span>(<span class="kw">super</span>) <span class="kw">fn </span>short_month0(s: <span class="kw-2">&amp;</span>str) -&gt; ParseResult&lt;(<span class="kw-2">&amp;</span>str, u8)&gt; {
    <span class="kw">if </span>s.len() &lt; <span class="number">3 </span>{
        <span class="kw">return </span><span class="prelude-val">Err</span>(TOO_SHORT);
    }
    <span class="kw">let </span>buf = s.as_bytes();
    <span class="kw">let </span>month0 = <span class="kw">match </span>(buf[<span class="number">0</span>] | <span class="number">32</span>, buf[<span class="number">1</span>] | <span class="number">32</span>, buf[<span class="number">2</span>] | <span class="number">32</span>) {
        (<span class="string">b'j'</span>, <span class="string">b'a'</span>, <span class="string">b'n'</span>) =&gt; <span class="number">0</span>,
        (<span class="string">b'f'</span>, <span class="string">b'e'</span>, <span class="string">b'b'</span>) =&gt; <span class="number">1</span>,
        (<span class="string">b'm'</span>, <span class="string">b'a'</span>, <span class="string">b'r'</span>) =&gt; <span class="number">2</span>,
        (<span class="string">b'a'</span>, <span class="string">b'p'</span>, <span class="string">b'r'</span>) =&gt; <span class="number">3</span>,
        (<span class="string">b'm'</span>, <span class="string">b'a'</span>, <span class="string">b'y'</span>) =&gt; <span class="number">4</span>,
        (<span class="string">b'j'</span>, <span class="string">b'u'</span>, <span class="string">b'n'</span>) =&gt; <span class="number">5</span>,
        (<span class="string">b'j'</span>, <span class="string">b'u'</span>, <span class="string">b'l'</span>) =&gt; <span class="number">6</span>,
        (<span class="string">b'a'</span>, <span class="string">b'u'</span>, <span class="string">b'g'</span>) =&gt; <span class="number">7</span>,
        (<span class="string">b's'</span>, <span class="string">b'e'</span>, <span class="string">b'p'</span>) =&gt; <span class="number">8</span>,
        (<span class="string">b'o'</span>, <span class="string">b'c'</span>, <span class="string">b't'</span>) =&gt; <span class="number">9</span>,
        (<span class="string">b'n'</span>, <span class="string">b'o'</span>, <span class="string">b'v'</span>) =&gt; <span class="number">10</span>,
        (<span class="string">b'd'</span>, <span class="string">b'e'</span>, <span class="string">b'c'</span>) =&gt; <span class="number">11</span>,
        <span class="kw">_ </span>=&gt; <span class="kw">return </span><span class="prelude-val">Err</span>(INVALID),
    };
    <span class="prelude-val">Ok</span>((<span class="kw-2">&amp;</span>s[<span class="number">3</span>..], month0))
}

<span class="doccomment">/// Tries to parse the weekday with the first three ASCII letters.
</span><span class="kw">pub</span>(<span class="kw">super</span>) <span class="kw">fn </span>short_weekday(s: <span class="kw-2">&amp;</span>str) -&gt; ParseResult&lt;(<span class="kw-2">&amp;</span>str, Weekday)&gt; {
    <span class="kw">if </span>s.len() &lt; <span class="number">3 </span>{
        <span class="kw">return </span><span class="prelude-val">Err</span>(TOO_SHORT);
    }
    <span class="kw">let </span>buf = s.as_bytes();
    <span class="kw">let </span>weekday = <span class="kw">match </span>(buf[<span class="number">0</span>] | <span class="number">32</span>, buf[<span class="number">1</span>] | <span class="number">32</span>, buf[<span class="number">2</span>] | <span class="number">32</span>) {
        (<span class="string">b'm'</span>, <span class="string">b'o'</span>, <span class="string">b'n'</span>) =&gt; Weekday::Mon,
        (<span class="string">b't'</span>, <span class="string">b'u'</span>, <span class="string">b'e'</span>) =&gt; Weekday::Tue,
        (<span class="string">b'w'</span>, <span class="string">b'e'</span>, <span class="string">b'd'</span>) =&gt; Weekday::Wed,
        (<span class="string">b't'</span>, <span class="string">b'h'</span>, <span class="string">b'u'</span>) =&gt; Weekday::Thu,
        (<span class="string">b'f'</span>, <span class="string">b'r'</span>, <span class="string">b'i'</span>) =&gt; Weekday::Fri,
        (<span class="string">b's'</span>, <span class="string">b'a'</span>, <span class="string">b't'</span>) =&gt; Weekday::Sat,
        (<span class="string">b's'</span>, <span class="string">b'u'</span>, <span class="string">b'n'</span>) =&gt; Weekday::Sun,
        <span class="kw">_ </span>=&gt; <span class="kw">return </span><span class="prelude-val">Err</span>(INVALID),
    };
    <span class="prelude-val">Ok</span>((<span class="kw-2">&amp;</span>s[<span class="number">3</span>..], weekday))
}

<span class="doccomment">/// Tries to parse the month index (0 through 11) with short or long month names.
/// It prefers long month names to short month names when both are possible.
</span><span class="kw">pub</span>(<span class="kw">super</span>) <span class="kw">fn </span>short_or_long_month0(s: <span class="kw-2">&amp;</span>str) -&gt; ParseResult&lt;(<span class="kw-2">&amp;</span>str, u8)&gt; {
    <span class="comment">// lowercased month names, minus first three chars
    </span><span class="kw">static </span>LONG_MONTH_SUFFIXES: [<span class="kw-2">&amp;</span>[u8]; <span class="number">12</span>] = [
        <span class="string">b"uary"</span>, <span class="string">b"ruary"</span>, <span class="string">b"ch"</span>, <span class="string">b"il"</span>, <span class="string">b""</span>, <span class="string">b"e"</span>, <span class="string">b"y"</span>, <span class="string">b"ust"</span>, <span class="string">b"tember"</span>, <span class="string">b"ober"</span>, <span class="string">b"ember"</span>,
        <span class="string">b"ember"</span>,
    ];

    <span class="kw">let </span>(<span class="kw-2">mut </span>s, month0) = short_month0(s)<span class="question-mark">?</span>;

    <span class="comment">// tries to consume the suffix if possible
    </span><span class="kw">let </span>suffix = LONG_MONTH_SUFFIXES[month0 <span class="kw">as </span>usize];
    <span class="kw">if </span>s.len() &gt;= suffix.len() &amp;&amp; s.as_bytes()[..suffix.len()].eq_ignore_ascii_case(suffix) {
        s = <span class="kw-2">&amp;</span>s[suffix.len()..];
    }

    <span class="prelude-val">Ok</span>((s, month0))
}

<span class="doccomment">/// Tries to parse the weekday with short or long weekday names.
/// It prefers long weekday names to short weekday names when both are possible.
</span><span class="kw">pub</span>(<span class="kw">super</span>) <span class="kw">fn </span>short_or_long_weekday(s: <span class="kw-2">&amp;</span>str) -&gt; ParseResult&lt;(<span class="kw-2">&amp;</span>str, Weekday)&gt; {
    <span class="comment">// lowercased weekday names, minus first three chars
    </span><span class="kw">static </span>LONG_WEEKDAY_SUFFIXES: [<span class="kw-2">&amp;</span>[u8]; <span class="number">7</span>] =
        [<span class="string">b"day"</span>, <span class="string">b"sday"</span>, <span class="string">b"nesday"</span>, <span class="string">b"rsday"</span>, <span class="string">b"day"</span>, <span class="string">b"urday"</span>, <span class="string">b"day"</span>];

    <span class="kw">let </span>(<span class="kw-2">mut </span>s, weekday) = short_weekday(s)<span class="question-mark">?</span>;

    <span class="comment">// tries to consume the suffix if possible
    </span><span class="kw">let </span>suffix = LONG_WEEKDAY_SUFFIXES[weekday.num_days_from_monday() <span class="kw">as </span>usize];
    <span class="kw">if </span>s.len() &gt;= suffix.len() &amp;&amp; s.as_bytes()[..suffix.len()].eq_ignore_ascii_case(suffix) {
        s = <span class="kw-2">&amp;</span>s[suffix.len()..];
    }

    <span class="prelude-val">Ok</span>((s, weekday))
}

<span class="doccomment">/// Tries to consume exactly one given character.
</span><span class="kw">pub</span>(<span class="kw">super</span>) <span class="kw">fn </span>char(s: <span class="kw-2">&amp;</span>str, c1: u8) -&gt; ParseResult&lt;<span class="kw-2">&amp;</span>str&gt; {
    <span class="kw">match </span>s.as_bytes().first() {
        <span class="prelude-val">Some</span>(<span class="kw-2">&amp;</span>c) <span class="kw">if </span>c == c1 =&gt; <span class="prelude-val">Ok</span>(<span class="kw-2">&amp;</span>s[<span class="number">1</span>..]),
        <span class="prelude-val">Some</span>(<span class="kw">_</span>) =&gt; <span class="prelude-val">Err</span>(INVALID),
        <span class="prelude-val">None </span>=&gt; <span class="prelude-val">Err</span>(TOO_SHORT),
    }
}

<span class="doccomment">/// Tries to consume one or more whitespace.
</span><span class="kw">pub</span>(<span class="kw">super</span>) <span class="kw">fn </span>space(s: <span class="kw-2">&amp;</span>str) -&gt; ParseResult&lt;<span class="kw-2">&amp;</span>str&gt; {
    <span class="kw">let </span>s_ = s.trim_start();
    <span class="kw">if </span>s_.len() &lt; s.len() {
        <span class="prelude-val">Ok</span>(s_)
    } <span class="kw">else if </span>s.is_empty() {
        <span class="prelude-val">Err</span>(TOO_SHORT)
    } <span class="kw">else </span>{
        <span class="prelude-val">Err</span>(INVALID)
    }
}

<span class="doccomment">/// Consumes any number (including zero) of colon or spaces.
</span><span class="kw">pub</span>(<span class="kw">crate</span>) <span class="kw">fn </span>colon_or_space(s: <span class="kw-2">&amp;</span>str) -&gt; ParseResult&lt;<span class="kw-2">&amp;</span>str&gt; {
    <span class="prelude-val">Ok</span>(s.trim_start_matches(|c: char| c == <span class="string">':' </span>|| c.is_whitespace()))
}

<span class="doccomment">/// Parse a timezone from `s` and return the offset in seconds.
///
/// The `consume_colon` function is used to parse a mandatory or optional `:`
/// separator between hours offset and minutes offset.
///
/// The `allow_missing_minutes` flag allows the timezone minutes offset to be
/// missing from `s`.
///
/// The `allow_tz_minus_sign` flag allows the timezone offset negative character
/// to also be `−` MINUS SIGN (U+2212) in addition to the typical
/// ASCII-compatible `-` HYPHEN-MINUS (U+2D).
/// This is part of [RFC 3339 &amp; ISO 8601].
///
/// [RFC 3339 &amp; ISO 8601]: https://en.wikipedia.org/w/index.php?title=ISO_8601&amp;oldid=1114309368#Time_offsets_from_UTC
</span><span class="kw">pub</span>(<span class="kw">crate</span>) <span class="kw">fn </span>timezone_offset&lt;F&gt;(
    <span class="kw-2">mut </span>s: <span class="kw-2">&amp;</span>str,
    <span class="kw-2">mut </span>consume_colon: F,
    allow_zulu: bool,
    allow_missing_minutes: bool,
    allow_tz_minus_sign: bool,
) -&gt; ParseResult&lt;(<span class="kw-2">&amp;</span>str, i32)&gt;
<span class="kw">where
    </span>F: FnMut(<span class="kw-2">&amp;</span>str) -&gt; ParseResult&lt;<span class="kw-2">&amp;</span>str&gt;,
{
    <span class="kw">if </span>allow_zulu {
        <span class="kw">if let </span><span class="prelude-val">Some</span>(<span class="kw-2">&amp;</span><span class="string">b'Z' </span>| <span class="kw-2">&amp;</span><span class="string">b'z'</span>) = s.as_bytes().first() {
            <span class="kw">return </span><span class="prelude-val">Ok</span>((<span class="kw-2">&amp;</span>s[<span class="number">1</span>..], <span class="number">0</span>));
        }
    }

    <span class="kw">const fn </span>digits(s: <span class="kw-2">&amp;</span>str) -&gt; ParseResult&lt;(u8, u8)&gt; {
        <span class="kw">let </span>b = s.as_bytes();
        <span class="kw">if </span>b.len() &lt; <span class="number">2 </span>{
            <span class="prelude-val">Err</span>(TOO_SHORT)
        } <span class="kw">else </span>{
            <span class="prelude-val">Ok</span>((b[<span class="number">0</span>], b[<span class="number">1</span>]))
        }
    }
    <span class="kw">let </span>negative = <span class="kw">match </span>s.chars().next() {
        <span class="prelude-val">Some</span>(<span class="string">'+'</span>) =&gt; {
            <span class="comment">// PLUS SIGN (U+2B)
            </span>s = <span class="kw-2">&amp;</span>s[<span class="string">'+'</span>.len_utf8()..];

            <span class="bool-val">false
        </span>}
        <span class="prelude-val">Some</span>(<span class="string">'-'</span>) =&gt; {
            <span class="comment">// HYPHEN-MINUS (U+2D)
            </span>s = <span class="kw-2">&amp;</span>s[<span class="string">'-'</span>.len_utf8()..];

            <span class="bool-val">true
        </span>}
        <span class="prelude-val">Some</span>(<span class="string">'−'</span>) =&gt; {
            <span class="comment">// MINUS SIGN (U+2212)
            </span><span class="kw">if </span>!allow_tz_minus_sign {
                <span class="kw">return </span><span class="prelude-val">Err</span>(INVALID);
            }
            s = <span class="kw-2">&amp;</span>s[<span class="string">'−'</span>.len_utf8()..];

            <span class="bool-val">true
        </span>}
        <span class="prelude-val">Some</span>(<span class="kw">_</span>) =&gt; <span class="kw">return </span><span class="prelude-val">Err</span>(INVALID),
        <span class="prelude-val">None </span>=&gt; <span class="kw">return </span><span class="prelude-val">Err</span>(TOO_SHORT),
    };

    <span class="comment">// hours (00--99)
    </span><span class="kw">let </span>hours = <span class="kw">match </span>digits(s)<span class="question-mark">? </span>{
        (h1 @ <span class="string">b'0'</span>..=<span class="string">b'9'</span>, h2 @ <span class="string">b'0'</span>..=<span class="string">b'9'</span>) =&gt; i32::from((h1 - <span class="string">b'0'</span>) * <span class="number">10 </span>+ (h2 - <span class="string">b'0'</span>)),
        <span class="kw">_ </span>=&gt; <span class="kw">return </span><span class="prelude-val">Err</span>(INVALID),
    };
    s = <span class="kw-2">&amp;</span>s[<span class="number">2</span>..];

    <span class="comment">// colons (and possibly other separators)
    </span>s = consume_colon(s)<span class="question-mark">?</span>;

    <span class="comment">// minutes (00--59)
    // if the next two items are digits then we have to add minutes
    </span><span class="kw">let </span>minutes = <span class="kw">if let </span><span class="prelude-val">Ok</span>(ds) = digits(s) {
        <span class="kw">match </span>ds {
            (m1 @ <span class="string">b'0'</span>..=<span class="string">b'5'</span>, m2 @ <span class="string">b'0'</span>..=<span class="string">b'9'</span>) =&gt; i32::from((m1 - <span class="string">b'0'</span>) * <span class="number">10 </span>+ (m2 - <span class="string">b'0'</span>)),
            (<span class="string">b'6'</span>..=<span class="string">b'9'</span>, <span class="string">b'0'</span>..=<span class="string">b'9'</span>) =&gt; <span class="kw">return </span><span class="prelude-val">Err</span>(OUT_OF_RANGE),
            <span class="kw">_ </span>=&gt; <span class="kw">return </span><span class="prelude-val">Err</span>(INVALID),
        }
    } <span class="kw">else if </span>allow_missing_minutes {
        <span class="number">0
    </span>} <span class="kw">else </span>{
        <span class="kw">return </span><span class="prelude-val">Err</span>(TOO_SHORT);
    };
    s = <span class="kw">match </span>s.len() {
        len <span class="kw">if </span>len &gt;= <span class="number">2 </span>=&gt; <span class="kw-2">&amp;</span>s[<span class="number">2</span>..],
        <span class="number">0 </span>=&gt; s,
        <span class="kw">_ </span>=&gt; <span class="kw">return </span><span class="prelude-val">Err</span>(TOO_SHORT),
    };

    <span class="kw">let </span>seconds = hours * <span class="number">3600 </span>+ minutes * <span class="number">60</span>;
    <span class="prelude-val">Ok</span>((s, <span class="kw">if </span>negative { -seconds } <span class="kw">else </span>{ seconds }))
}

<span class="doccomment">/// Same as `timezone_offset` but also allows for RFC 2822 legacy timezones.
/// May return `None` which indicates an insufficient offset data (i.e. `-0000`).
/// See [RFC 2822 Section 4.3].
///
/// [RFC 2822 Section 4.3]: https://tools.ietf.org/html/rfc2822#section-4.3
</span><span class="kw">pub</span>(<span class="kw">super</span>) <span class="kw">fn </span>timezone_offset_2822(s: <span class="kw-2">&amp;</span>str) -&gt; ParseResult&lt;(<span class="kw-2">&amp;</span>str, i32)&gt; {
    <span class="comment">// tries to parse legacy time zone names
    </span><span class="kw">let </span>upto = s.as_bytes().iter().position(|<span class="kw-2">&amp;</span>c| !c.is_ascii_alphabetic()).unwrap_or(s.len());
    <span class="kw">if </span>upto &gt; <span class="number">0 </span>{
        <span class="kw">let </span>name = <span class="kw-2">&amp;</span>s.as_bytes()[..upto];
        <span class="kw">let </span>s = <span class="kw-2">&amp;</span>s[upto..];
        <span class="kw">let </span>offset_hours = |o| <span class="prelude-val">Ok</span>((s, o * <span class="number">3600</span>));
        <span class="comment">// RFC 2822 requires support for some named North America timezones, a small subset of all
        // named timezones.
        </span><span class="kw">if </span>name.eq_ignore_ascii_case(<span class="string">b"gmt"</span>)
            || name.eq_ignore_ascii_case(<span class="string">b"ut"</span>)
            || name.eq_ignore_ascii_case(<span class="string">b"z"</span>)
        {
            <span class="kw">return </span>offset_hours(<span class="number">0</span>);
        } <span class="kw">else if </span>name.eq_ignore_ascii_case(<span class="string">b"edt"</span>) {
            <span class="kw">return </span>offset_hours(-<span class="number">4</span>);
        } <span class="kw">else if </span>name.eq_ignore_ascii_case(<span class="string">b"est"</span>) || name.eq_ignore_ascii_case(<span class="string">b"cdt"</span>) {
            <span class="kw">return </span>offset_hours(-<span class="number">5</span>);
        } <span class="kw">else if </span>name.eq_ignore_ascii_case(<span class="string">b"cst"</span>) || name.eq_ignore_ascii_case(<span class="string">b"mdt"</span>) {
            <span class="kw">return </span>offset_hours(-<span class="number">6</span>);
        } <span class="kw">else if </span>name.eq_ignore_ascii_case(<span class="string">b"mst"</span>) || name.eq_ignore_ascii_case(<span class="string">b"pdt"</span>) {
            <span class="kw">return </span>offset_hours(-<span class="number">7</span>);
        } <span class="kw">else if </span>name.eq_ignore_ascii_case(<span class="string">b"pst"</span>) {
            <span class="kw">return </span>offset_hours(-<span class="number">8</span>);
        } <span class="kw">else if </span>name.len() == <span class="number">1 </span>{
            <span class="kw">if let </span><span class="string">b'a'</span>..=<span class="string">b'i' </span>| <span class="string">b'k'</span>..=<span class="string">b'y' </span>| <span class="string">b'A'</span>..=<span class="string">b'I' </span>| <span class="string">b'K'</span>..=<span class="string">b'Y' </span>= name[<span class="number">0</span>] {
                <span class="comment">// recommended by RFC 2822: consume but treat it as -0000
                </span><span class="kw">return </span><span class="prelude-val">Ok</span>((s, <span class="number">0</span>));
            }
        }
        <span class="prelude-val">Err</span>(INVALID)
    } <span class="kw">else </span>{
        timezone_offset(s, |s| <span class="prelude-val">Ok</span>(s), <span class="bool-val">false</span>, <span class="bool-val">false</span>, <span class="bool-val">false</span>)
    }
}

<span class="doccomment">/// Tries to consume an RFC2822 comment including preceding ` `.
///
/// Returns the remaining string after the closing parenthesis.
</span><span class="kw">pub</span>(<span class="kw">super</span>) <span class="kw">fn </span>comment_2822(s: <span class="kw-2">&amp;</span>str) -&gt; ParseResult&lt;(<span class="kw-2">&amp;</span>str, ())&gt; {
    <span class="kw">use </span>CommentState::<span class="kw-2">*</span>;

    <span class="kw">let </span>s = s.trim_start();

    <span class="kw">let </span><span class="kw-2">mut </span>state = Start;
    <span class="kw">for </span>(i, c) <span class="kw">in </span>s.bytes().enumerate() {
        state = <span class="kw">match </span>(state, c) {
            (Start, <span class="string">b'('</span>) =&gt; Next(<span class="number">1</span>),
            (Next(<span class="number">1</span>), <span class="string">b')'</span>) =&gt; <span class="kw">return </span><span class="prelude-val">Ok</span>((<span class="kw-2">&amp;</span>s[i + <span class="number">1</span>..], ())),
            (Next(depth), <span class="string">b'\\'</span>) =&gt; Escape(depth),
            (Next(depth), <span class="string">b'('</span>) =&gt; Next(depth + <span class="number">1</span>),
            (Next(depth), <span class="string">b')'</span>) =&gt; Next(depth - <span class="number">1</span>),
            (Next(depth), <span class="kw">_</span>) | (Escape(depth), <span class="kw">_</span>) =&gt; Next(depth),
            <span class="kw">_ </span>=&gt; <span class="kw">return </span><span class="prelude-val">Err</span>(INVALID),
        };
    }

    <span class="prelude-val">Err</span>(TOO_SHORT)
}

<span class="kw">enum </span>CommentState {
    Start,
    Next(usize),
    Escape(usize),
}

<span class="attr">#[cfg(test)]
</span><span class="kw">mod </span>tests {
    <span class="kw">use super</span>::{
        comment_2822, nanosecond, nanosecond_fixed, short_or_long_month0, short_or_long_weekday,
        timezone_offset_2822,
    };
    <span class="kw">use </span><span class="kw">crate</span>::format::{INVALID, TOO_SHORT};
    <span class="kw">use </span><span class="kw">crate</span>::Weekday;

    <span class="attr">#[test]
    </span><span class="kw">fn </span>test_rfc2822_comments() {
        <span class="kw">let </span>testdata = [
            (<span class="string">""</span>, <span class="prelude-val">Err</span>(TOO_SHORT)),
            (<span class="string">" "</span>, <span class="prelude-val">Err</span>(TOO_SHORT)),
            (<span class="string">"x"</span>, <span class="prelude-val">Err</span>(INVALID)),
            (<span class="string">"("</span>, <span class="prelude-val">Err</span>(TOO_SHORT)),
            (<span class="string">"()"</span>, <span class="prelude-val">Ok</span>(<span class="string">""</span>)),
            (<span class="string">" \r\n\t()"</span>, <span class="prelude-val">Ok</span>(<span class="string">""</span>)),
            (<span class="string">"() "</span>, <span class="prelude-val">Ok</span>(<span class="string">" "</span>)),
            (<span class="string">"()z"</span>, <span class="prelude-val">Ok</span>(<span class="string">"z"</span>)),
            (<span class="string">"(x)"</span>, <span class="prelude-val">Ok</span>(<span class="string">""</span>)),
            (<span class="string">"(())"</span>, <span class="prelude-val">Ok</span>(<span class="string">""</span>)),
            (<span class="string">"((()))"</span>, <span class="prelude-val">Ok</span>(<span class="string">""</span>)),
            (<span class="string">"(x(x(x)x)x)"</span>, <span class="prelude-val">Ok</span>(<span class="string">""</span>)),
            (<span class="string">"( x ( x ( x ) x ) x )"</span>, <span class="prelude-val">Ok</span>(<span class="string">""</span>)),
            (<span class="string">r"(\)"</span>, <span class="prelude-val">Err</span>(TOO_SHORT)),
            (<span class="string">r"(\()"</span>, <span class="prelude-val">Ok</span>(<span class="string">""</span>)),
            (<span class="string">r"(\))"</span>, <span class="prelude-val">Ok</span>(<span class="string">""</span>)),
            (<span class="string">r"(\\)"</span>, <span class="prelude-val">Ok</span>(<span class="string">""</span>)),
            (<span class="string">"(()())"</span>, <span class="prelude-val">Ok</span>(<span class="string">""</span>)),
            (<span class="string">"( x ( x ) x ( x ) x )"</span>, <span class="prelude-val">Ok</span>(<span class="string">""</span>)),
        ];

        <span class="kw">for </span>(test_in, expected) <span class="kw">in </span>testdata.iter() {
            <span class="kw">let </span>actual = comment_2822(test_in).map(|(s, <span class="kw">_</span>)| s);
            <span class="macro">assert_eq!</span>(
                <span class="kw-2">*</span>expected, actual,
                <span class="string">"{:?} expected to produce {:?}, but produced {:?}."</span>,
                test_in, expected, actual
            );
        }
    }

    <span class="attr">#[test]
    </span><span class="kw">fn </span>test_timezone_offset_2822() {
        <span class="macro">assert_eq!</span>(timezone_offset_2822(<span class="string">"cSt"</span>).unwrap(), (<span class="string">""</span>, -<span class="number">21600</span>));
        <span class="macro">assert_eq!</span>(timezone_offset_2822(<span class="string">"pSt"</span>).unwrap(), (<span class="string">""</span>, -<span class="number">28800</span>));
        <span class="macro">assert_eq!</span>(timezone_offset_2822(<span class="string">"mSt"</span>).unwrap(), (<span class="string">""</span>, -<span class="number">25200</span>));
        <span class="macro">assert_eq!</span>(timezone_offset_2822(<span class="string">"-1551"</span>).unwrap(), (<span class="string">""</span>, -<span class="number">57060</span>));
        <span class="macro">assert_eq!</span>(timezone_offset_2822(<span class="string">"Gp"</span>), <span class="prelude-val">Err</span>(INVALID));
    }

    <span class="attr">#[test]
    </span><span class="kw">fn </span>test_short_or_long_month0() {
        <span class="macro">assert_eq!</span>(short_or_long_month0(<span class="string">"JUn"</span>).unwrap(), (<span class="string">""</span>, <span class="number">5</span>));
        <span class="macro">assert_eq!</span>(short_or_long_month0(<span class="string">"mAy"</span>).unwrap(), (<span class="string">""</span>, <span class="number">4</span>));
        <span class="macro">assert_eq!</span>(short_or_long_month0(<span class="string">"AuG"</span>).unwrap(), (<span class="string">""</span>, <span class="number">7</span>));
        <span class="macro">assert_eq!</span>(short_or_long_month0(<span class="string">"Aprâ"</span>).unwrap(), (<span class="string">"â"</span>, <span class="number">3</span>));
        <span class="macro">assert_eq!</span>(short_or_long_month0(<span class="string">"JUl"</span>).unwrap(), (<span class="string">""</span>, <span class="number">6</span>));
        <span class="macro">assert_eq!</span>(short_or_long_month0(<span class="string">"mAr"</span>).unwrap(), (<span class="string">""</span>, <span class="number">2</span>));
        <span class="macro">assert_eq!</span>(short_or_long_month0(<span class="string">"Jan"</span>).unwrap(), (<span class="string">""</span>, <span class="number">0</span>));
    }

    <span class="attr">#[test]
    </span><span class="kw">fn </span>test_short_or_long_weekday() {
        <span class="macro">assert_eq!</span>(short_or_long_weekday(<span class="string">"sAtu"</span>).unwrap(), (<span class="string">"u"</span>, Weekday::Sat));
        <span class="macro">assert_eq!</span>(short_or_long_weekday(<span class="string">"thu"</span>).unwrap(), (<span class="string">""</span>, Weekday::Thu));
    }

    <span class="attr">#[test]
    </span><span class="kw">fn </span>test_nanosecond_fixed() {
        <span class="macro">assert_eq!</span>(nanosecond_fixed(<span class="string">""</span>, <span class="number">0usize</span>).unwrap(), (<span class="string">""</span>, <span class="number">0</span>));
        <span class="macro">assert!</span>(nanosecond_fixed(<span class="string">""</span>, <span class="number">1usize</span>).is_err());
    }

    <span class="attr">#[test]
    </span><span class="kw">fn </span>test_nanosecond() {
        <span class="macro">assert_eq!</span>(nanosecond(<span class="string">"2Ù"</span>).unwrap(), (<span class="string">"Ù"</span>, <span class="number">200000000</span>));
        <span class="macro">assert_eq!</span>(nanosecond(<span class="string">"8"</span>).unwrap(), (<span class="string">""</span>, <span class="number">800000000</span>));
    }
}
</code></pre></div></section></main></body></html>