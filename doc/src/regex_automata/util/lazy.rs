<!DOCTYPE html><html lang="en"><head><meta charset="utf-8"><meta name="viewport" content="width=device-width, initial-scale=1.0"><meta name="generator" content="rustdoc"><meta name="description" content="Source of the Rust file `/Users/jeyakumar/.cargo/registry/src/index.crates.io-6f17d22bba15001f/regex-automata-0.4.6/src/util/lazy.rs`."><title>lazy.rs - source</title><script>if(window.location.protocol!=="file:")document.head.insertAdjacentHTML("beforeend","SourceSerif4-Regular-46f98efaafac5295.ttf.woff2,FiraSans-Regular-018c141bf0843ffd.woff2,FiraSans-Medium-8f9a781e4970d388.woff2,SourceCodePro-Regular-562dcc5011b6de7d.ttf.woff2,SourceCodePro-Semibold-d899c5a5c4aeb14a.ttf.woff2".split(",").map(f=>`<link rel="preload" as="font" type="font/woff2" crossorigin href="../../../static.files/${f}">`).join(""))</script><link rel="stylesheet" href="../../../static.files/normalize-76eba96aa4d2e634.css"><link rel="stylesheet" href="../../../static.files/rustdoc-081576b923113409.css"><meta name="rustdoc-vars" data-root-path="../../../" data-static-root-path="../../../static.files/" data-current-crate="regex_automata" data-themes="" data-resource-suffix="" data-rustdoc-version="1.79.0 (129f3b996 2024-06-10)" data-channel="1.79.0" data-search-js="search-bf21c90c8c1d92b1.js" data-settings-js="settings-4313503d2e1961c2.js" ><script src="../../../static.files/storage-e32f0c247825364d.js"></script><script defer src="../../../static.files/src-script-e66d777a5a92e9b2.js"></script><script defer src="../../../src-files.js"></script><script defer src="../../../static.files/main-20a3ad099b048cf2.js"></script><noscript><link rel="stylesheet" href="../../../static.files/noscript-09095024cf37855e.css"></noscript><link rel="alternate icon" type="image/png" href="../../../static.files/favicon-32x32-422f7d1d52889060.png"><link rel="icon" type="image/svg+xml" href="../../../static.files/favicon-2c020d218678b618.svg"></head><body class="rustdoc src"><!--[if lte IE 11]><div class="warning">This old browser is unsupported and will most likely display funky things.</div><![endif]--><nav class="sidebar"><div class="src-sidebar-title"><h2>Files</h2></div></nav><div class="sidebar-resizer"></div><main><nav class="sub"><form class="search-form"><span></span><div id="sidebar-button" tabindex="-1"><a href="../../../regex_automata/all.html" title="show sidebar"></a></div><input class="search-input" name="search" aria-label="Run search in the documentation" autocomplete="off" spellcheck="false" placeholder="Type ‘S’ or ‘/’ to search, ‘?’ for more options…" type="search"><div id="help-button" tabindex="-1"><a href="../../../help.html" title="help">?</a></div><div id="settings-menu" tabindex="-1"><a href="../../../settings.html" title="settings">Settings</a></div></form></nav><section id="main-content" class="content"><div class="example-wrap"><div data-nosnippet><pre class="src-line-numbers"><a href="#1" id="1">1</a>
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
<a href="#437" id="437">437</a>
<a href="#438" id="438">438</a>
<a href="#439" id="439">439</a>
<a href="#440" id="440">440</a>
<a href="#441" id="441">441</a>
<a href="#442" id="442">442</a>
<a href="#443" id="443">443</a>
<a href="#444" id="444">444</a>
<a href="#445" id="445">445</a>
<a href="#446" id="446">446</a>
<a href="#447" id="447">447</a>
<a href="#448" id="448">448</a>
<a href="#449" id="449">449</a>
<a href="#450" id="450">450</a>
<a href="#451" id="451">451</a>
<a href="#452" id="452">452</a>
<a href="#453" id="453">453</a>
<a href="#454" id="454">454</a>
<a href="#455" id="455">455</a>
<a href="#456" id="456">456</a>
<a href="#457" id="457">457</a>
<a href="#458" id="458">458</a>
<a href="#459" id="459">459</a>
<a href="#460" id="460">460</a>
<a href="#461" id="461">461</a>
</pre></div><pre class="rust"><code><span class="doccomment">/*!
A lazily initialized value for safe sharing between threads.

The principal type in this module is `Lazy`, which makes it easy to construct
values that are shared safely across multiple threads simultaneously.
*/

</span><span class="kw">use </span>core::fmt;

<span class="doccomment">/// A lazily initialized value that implements `Deref` for `T`.
///
/// A `Lazy` takes an initialization function and permits callers from any
/// thread to access the result of that initialization function in a safe
/// manner. In effect, this permits one-time initialization of global resources
/// in a (possibly) multi-threaded program.
///
/// This type and its functionality are available even when neither the `alloc`
/// nor the `std` features are enabled. In exchange, a `Lazy` does **not**
/// guarantee that the given `create` function is called at most once. It
/// might be called multiple times. Moreover, a call to `Lazy::get` (either
/// explicitly or implicitly via `Lazy`'s `Deref` impl) may block until a `T`
/// is available.
///
/// This is very similar to `lazy_static` or `once_cell`, except it doesn't
/// guarantee that the initialization function will be run once and it works
/// in no-alloc no-std environments. With that said, if you need stronger
/// guarantees or a more flexible API, then it is recommended to use either
/// `lazy_static` or `once_cell`.
///
/// # Warning: may use a spin lock
///
/// When this crate is compiled _without_ the `alloc` feature, then this type
/// may used a spin lock internally. This can have subtle effects that may
/// be undesirable. See [Spinlocks Considered Harmful][spinharm] for a more
/// thorough treatment of this topic.
///
/// [spinharm]: https://matklad.github.io/2020/01/02/spinlocks-considered-harmful.html
///
/// # Example
///
/// This type is useful for creating regexes once, and then using them from
/// multiple threads simultaneously without worrying about synchronization.
///
/// ```
/// use regex_automata::{dfa::regex::Regex, util::lazy::Lazy, Match};
///
/// static RE: Lazy&lt;Regex&gt; = Lazy::new(|| Regex::new("foo[0-9]+bar").unwrap());
///
/// let expected = Some(Match::must(0, 3..14));
/// assert_eq!(expected, RE.find(b"zzzfoo12345barzzz"));
/// ```
</span><span class="kw">pub struct </span>Lazy&lt;T, F = <span class="kw">fn</span>() -&gt; T&gt;(lazy::Lazy&lt;T, F&gt;);

<span class="kw">impl</span>&lt;T, F&gt; Lazy&lt;T, F&gt; {
    <span class="doccomment">/// Create a new `Lazy` value that is initialized via the given function.
    ///
    /// The `T` type is automatically inferred from the return type of the
    /// `create` function given.
    </span><span class="kw">pub const fn </span>new(create: F) -&gt; Lazy&lt;T, F&gt; {
        Lazy(lazy::Lazy::new(create))
    }
}

<span class="kw">impl</span>&lt;T, F: Fn() -&gt; T&gt; Lazy&lt;T, F&gt; {
    <span class="doccomment">/// Return a reference to the lazily initialized value.
    ///
    /// This routine may block if another thread is initializing a `T`.
    ///
    /// Note that given a `x` which has type `Lazy`, this must be called via
    /// `Lazy::get(x)` and not `x.get()`. This routine is defined this way
    /// because `Lazy` impls `Deref` with a target of `T`.
    ///
    /// # Panics
    ///
    /// This panics if the `create` function inside this lazy value panics.
    /// If the panic occurred in another thread, then this routine _may_ also
    /// panic (but is not guaranteed to do so).
    </span><span class="kw">pub fn </span>get(this: <span class="kw-2">&amp;</span>Lazy&lt;T, F&gt;) -&gt; <span class="kw-2">&amp;</span>T {
        this.<span class="number">0</span>.get()
    }
}

<span class="kw">impl</span>&lt;T, F: Fn() -&gt; T&gt; core::ops::Deref <span class="kw">for </span>Lazy&lt;T, F&gt; {
    <span class="kw">type </span>Target = T;

    <span class="kw">fn </span>deref(<span class="kw-2">&amp;</span><span class="self">self</span>) -&gt; <span class="kw-2">&amp;</span>T {
        Lazy::get(<span class="self">self</span>)
    }
}

<span class="kw">impl</span>&lt;T: fmt::Debug, F: Fn() -&gt; T&gt; fmt::Debug <span class="kw">for </span>Lazy&lt;T, F&gt; {
    <span class="kw">fn </span>fmt(<span class="kw-2">&amp;</span><span class="self">self</span>, f: <span class="kw-2">&amp;mut </span>fmt::Formatter) -&gt; fmt::Result {
        <span class="self">self</span>.<span class="number">0</span>.fmt(f)
    }
}

<span class="attr">#[cfg(feature = <span class="string">"alloc"</span>)]
</span><span class="kw">mod </span>lazy {
    <span class="kw">use </span>core::{
        fmt,
        marker::PhantomData,
        sync::atomic::{AtomicPtr, Ordering},
    };

    <span class="kw">use </span>alloc::boxed::Box;

    <span class="doccomment">/// A non-std lazy initialized value.
    ///
    /// This might run the initialization function more than once, but will
    /// never block.
    ///
    /// I wish I could get these semantics into the non-alloc non-std Lazy
    /// type below, but I'm not sure how to do it. If you can do an alloc,
    /// then the implementation becomes very simple if you don't care about
    /// redundant work precisely because a pointer can be atomically swapped.
    ///
    /// Perhaps making this approach work in the non-alloc non-std case
    /// requires asking the caller for a pointer? It would make the API less
    /// convenient I think.
    </span><span class="kw">pub</span>(<span class="kw">super</span>) <span class="kw">struct </span>Lazy&lt;T, F&gt; {
        data: AtomicPtr&lt;T&gt;,
        create: F,
        <span class="comment">// This indicates to the compiler that this type can drop T. It's not
        // totally clear how the absence of this marker could lead to trouble,
        // but putting here doesn't have any downsides so we hedge until somone
        // can from the Unsafe Working Group can tell us definitively that we
        // don't need it.
        //
        // See: https://github.com/BurntSushi/regex-automata/issues/30
        </span>owned: PhantomData&lt;Box&lt;T&gt;&gt;,
    }

    <span class="comment">// SAFETY: So long as T and &amp;T (and F and &amp;F) can themselves be safely
    // shared among threads, so to can a Lazy&lt;T, _&gt;. Namely, the Lazy API only
    // permits accessing a &amp;T and initialization is free of data races. So if T
    // is thread safe, then so to is Lazy&lt;T, _&gt;.
    //
    // We specifically require that T: Send in order for Lazy&lt;T&gt; to be Sync.
    // Without that requirement, it's possible to send a T from one thread to
    // another via Lazy's destructor.
    //
    // It's not clear whether we need F: Send+Sync for Lazy to be Sync. But
    // we're conservative for now and keep both.
    </span><span class="kw">unsafe impl</span>&lt;T: Send + Sync, F: Send + Sync&gt; Sync <span class="kw">for </span>Lazy&lt;T, F&gt; {}

    <span class="kw">impl</span>&lt;T, F&gt; Lazy&lt;T, F&gt; {
        <span class="doccomment">/// Create a new alloc but non-std lazy value that is racily
        /// initialized. That is, the 'create' function may be called more than
        /// once.
        </span><span class="kw">pub</span>(<span class="kw">super</span>) <span class="kw">const fn </span>new(create: F) -&gt; Lazy&lt;T, F&gt; {
            Lazy {
                data: AtomicPtr::new(core::ptr::null_mut()),
                create,
                owned: PhantomData,
            }
        }
    }

    <span class="kw">impl</span>&lt;T, F: Fn() -&gt; T&gt; Lazy&lt;T, F&gt; {
        <span class="doccomment">/// Get the underlying lazy value. If it hasn't been initialized
        /// yet, then always attempt to initialize it (even if some other
        /// thread is initializing it) and atomically attach it to this lazy
        /// value before returning it.
        </span><span class="kw">pub</span>(<span class="kw">super</span>) <span class="kw">fn </span>get(<span class="kw-2">&amp;</span><span class="self">self</span>) -&gt; <span class="kw-2">&amp;</span>T {
            <span class="kw">if let </span><span class="prelude-val">Some</span>(data) = <span class="self">self</span>.poll() {
                <span class="kw">return </span>data;
            }
            <span class="kw">let </span>data = (<span class="self">self</span>.create)();
            <span class="kw">let </span><span class="kw-2">mut </span>ptr = Box::into_raw(Box::new(data));
            <span class="comment">// We attempt to stuff our initialized value into our atomic
            // pointer. Upon success, we don't need to do anything. But if
            // someone else beat us to the punch, then we need to make sure
            // our newly created value is dropped.
            </span><span class="kw">let </span>result = <span class="self">self</span>.data.compare_exchange(
                core::ptr::null_mut(),
                ptr,
                Ordering::AcqRel,
                Ordering::Acquire,
            );
            <span class="kw">if let </span><span class="prelude-val">Err</span>(old) = result {
                <span class="comment">// SAFETY: We created 'ptr' via Box::into_raw above, so turning
                // it back into a Box via from_raw is safe.
                </span>drop(<span class="kw">unsafe </span>{ Box::from_raw(ptr) });
                ptr = old;
            }
            <span class="comment">// SAFETY: We just set the pointer above to a non-null value, even
            // in the error case, and set it to a fully initialized value
            // returned by 'create'.
            </span><span class="kw">unsafe </span>{ <span class="kw-2">&amp;*</span>ptr }
        }

        <span class="doccomment">/// If this lazy value has been initialized successfully, then return
        /// that value. Otherwise return None immediately. This never attempts
        /// to run initialization itself.
        </span><span class="kw">fn </span>poll(<span class="kw-2">&amp;</span><span class="self">self</span>) -&gt; <span class="prelude-ty">Option</span>&lt;<span class="kw-2">&amp;</span>T&gt; {
            <span class="kw">let </span>ptr = <span class="self">self</span>.data.load(Ordering::Acquire);
            <span class="kw">if </span>ptr.is_null() {
                <span class="kw">return </span><span class="prelude-val">None</span>;
            }
            <span class="comment">// SAFETY: We just checked that the pointer is not null. Since it's
            // not null, it must have been fully initialized by 'get' at some
            // point.
            </span><span class="prelude-val">Some</span>(<span class="kw">unsafe </span>{ <span class="kw-2">&amp;*</span>ptr })
        }
    }

    <span class="kw">impl</span>&lt;T: fmt::Debug, F: Fn() -&gt; T&gt; fmt::Debug <span class="kw">for </span>Lazy&lt;T, F&gt; {
        <span class="kw">fn </span>fmt(<span class="kw-2">&amp;</span><span class="self">self</span>, f: <span class="kw-2">&amp;mut </span>fmt::Formatter) -&gt; fmt::Result {
            f.debug_struct(<span class="string">"Lazy"</span>).field(<span class="string">"data"</span>, <span class="kw-2">&amp;</span><span class="self">self</span>.poll()).finish()
        }
    }

    <span class="kw">impl</span>&lt;T, F&gt; Drop <span class="kw">for </span>Lazy&lt;T, F&gt; {
        <span class="kw">fn </span>drop(<span class="kw-2">&amp;mut </span><span class="self">self</span>) {
            <span class="kw">let </span>ptr = <span class="kw-2">*</span><span class="self">self</span>.data.get_mut();
            <span class="kw">if </span>!ptr.is_null() {
                <span class="comment">// SAFETY: We just checked that 'ptr' is not null. And since
                // we have exclusive access, there are no races to worry about.
                </span>drop(<span class="kw">unsafe </span>{ Box::from_raw(ptr) });
            }
        }
    }
}

<span class="attr">#[cfg(not(feature = <span class="string">"alloc"</span>))]
</span><span class="kw">mod </span>lazy {
    <span class="kw">use </span>core::{
        cell::Cell,
        fmt,
        mem::MaybeUninit,
        panic::{RefUnwindSafe, UnwindSafe},
        sync::atomic::{AtomicU8, Ordering},
    };

    <span class="doccomment">/// Our 'Lazy' value can be in one of three states:
    ///
    /// * INIT is where it starts, and also ends up back here if the
    /// 'create' routine panics.
    /// * BUSY is where it sits while initialization is running in exactly
    /// one thread.
    /// * DONE is where it sits after 'create' has completed and 'data' has
    /// been fully initialized.
    </span><span class="kw">const </span>LAZY_STATE_INIT: u8 = <span class="number">0</span>;
    <span class="kw">const </span>LAZY_STATE_BUSY: u8 = <span class="number">1</span>;
    <span class="kw">const </span>LAZY_STATE_DONE: u8 = <span class="number">2</span>;

    <span class="doccomment">/// A non-alloc non-std lazy initialized value.
    ///
    /// This guarantees initialization only happens once, but uses a spinlock
    /// to block in the case of simultaneous access. Blocking occurs so that
    /// one thread waits while another thread initializes the value.
    ///
    /// I would much rather have the semantics of the 'alloc' Lazy type above.
    /// Namely, that we might run the initialization function more than once,
    /// but we never otherwise block. However, I don't know how to do that in
    /// a non-alloc non-std context.
    </span><span class="kw">pub</span>(<span class="kw">super</span>) <span class="kw">struct </span>Lazy&lt;T, F&gt; {
        state: AtomicU8,
        create: Cell&lt;<span class="prelude-ty">Option</span>&lt;F&gt;&gt;,
        data: Cell&lt;MaybeUninit&lt;T&gt;&gt;,
    }

    <span class="comment">// SAFETY: So long as T and &amp;T (and F and &amp;F) can themselves be safely
    // shared among threads, so to can a Lazy&lt;T, _&gt;. Namely, the Lazy API only
    // permits accessing a &amp;T and initialization is free of data races. So if T
    // is thread safe, then so to is Lazy&lt;T, _&gt;.
    </span><span class="kw">unsafe impl</span>&lt;T: Send + Sync, F: Send + Sync&gt; Sync <span class="kw">for </span>Lazy&lt;T, F&gt; {}
    <span class="comment">// A reference to a Lazy is unwind safe because we specifically take
    // precautions to poison all accesses to a Lazy if the caller-provided
    // 'create' function panics.
    </span><span class="kw">impl</span>&lt;T: UnwindSafe, F: UnwindSafe + RefUnwindSafe&gt; RefUnwindSafe
        <span class="kw">for </span>Lazy&lt;T, F&gt;
    {
    }

    <span class="kw">impl</span>&lt;T, F&gt; Lazy&lt;T, F&gt; {
        <span class="doccomment">/// Create a new non-alloc non-std lazy value that is initialized
        /// exactly once on first use using the given function.
        </span><span class="kw">pub</span>(<span class="kw">super</span>) <span class="kw">const fn </span>new(create: F) -&gt; Lazy&lt;T, F&gt; {
            Lazy {
                state: AtomicU8::new(LAZY_STATE_INIT),
                create: Cell::new(<span class="prelude-val">Some</span>(create)),
                data: Cell::new(MaybeUninit::uninit()),
            }
        }
    }

    <span class="kw">impl</span>&lt;T, F: FnOnce() -&gt; T&gt; Lazy&lt;T, F&gt; {
        <span class="doccomment">/// Get the underlying lazy value. If it isn't been initialized
        /// yet, then either initialize it or block until some other thread
        /// initializes it. If the 'create' function given to Lazy::new panics
        /// (even in another thread), then this panics too.
        </span><span class="kw">pub</span>(<span class="kw">super</span>) <span class="kw">fn </span>get(<span class="kw-2">&amp;</span><span class="self">self</span>) -&gt; <span class="kw-2">&amp;</span>T {
            <span class="comment">// This is effectively a spinlock. We loop until we enter a DONE
            // state, and if possible, initialize it ourselves. The only way
            // we exit the loop is if 'create' panics, we initialize 'data' or
            // some other thread initializes 'data'.
            //
            // Yes, I have read spinlocks considered harmful[1]. And that
            // article is why this spinlock is only active when 'alloc' isn't
            // enabled. I did this because I don't think there is really
            // another choice without 'alloc', other than not providing this at
            // all. But I think that's a big bummer.
            //
            // [1]: https://matklad.github.io/2020/01/02/spinlocks-considered-harmful.html
            </span><span class="kw">while </span><span class="self">self</span>.state.load(Ordering::Acquire) != LAZY_STATE_DONE {
                <span class="comment">// Check if we're the first ones to get here. If so, we'll be
                // the ones who initialize.
                </span><span class="kw">let </span>result = <span class="self">self</span>.state.compare_exchange(
                    LAZY_STATE_INIT,
                    LAZY_STATE_BUSY,
                    Ordering::AcqRel,
                    Ordering::Acquire,
                );
                <span class="comment">// This means we saw the INIT state and nobody else can. So we
                // must take responsibility for initializing. And by virtue of
                // observing INIT, we have also told anyone else trying to
                // get here that we are BUSY. If someone else sees BUSY, then
                // they will spin until we finish initialization.
                </span><span class="kw">if let </span><span class="prelude-val">Ok</span>(<span class="kw">_</span>) = result {
                    <span class="comment">// Since we are guaranteed to be the only ones here, we
                    // know that 'create' is there... Unless someone else got
                    // here before us and 'create' panicked. In which case,
                    // 'self.create' is now 'None' and we forward the panic
                    // to the caller. (i.e., We implement poisoning.)
                    //
                    // SAFETY: Our use of 'self.state' guarantees that we are
                    // the only thread executing this line, and thus there are
                    // no races.
                    </span><span class="kw">let </span>create = <span class="kw">unsafe </span>{
                        (<span class="kw-2">*</span><span class="self">self</span>.create.as_ptr()).take().expect(
                            <span class="string">"Lazy's create function panicked, \
                             preventing initialization,
                             poisoning current thread"</span>,
                        )
                    };
                    <span class="kw">let </span>guard = Guard { state: <span class="kw-2">&amp;</span><span class="self">self</span>.state };
                    <span class="comment">// SAFETY: Our use of 'self.state' guarantees that we are
                    // the only thread executing this line, and thus there are
                    // no races.
                    </span><span class="kw">unsafe </span>{
                        (<span class="kw-2">*</span><span class="self">self</span>.data.as_ptr()).as_mut_ptr().write(create());
                    }
                    <span class="comment">// All is well. 'self.create' ran successfully, so we
                    // forget the guard.
                    </span>core::mem::forget(guard);
                    <span class="comment">// Everything is initialized, so we can declare success.
                    </span><span class="self">self</span>.state.store(LAZY_STATE_DONE, Ordering::Release);
                    <span class="kw">break</span>;
                }
                core::hint::spin_loop();
            }
            <span class="comment">// We only get here if data is fully initialized, and thus poll
            // will always return something.
            </span><span class="self">self</span>.poll().unwrap()
        }

        <span class="doccomment">/// If this lazy value has been initialized successfully, then return
        /// that value. Otherwise return None immediately. This never blocks.
        </span><span class="kw">fn </span>poll(<span class="kw-2">&amp;</span><span class="self">self</span>) -&gt; <span class="prelude-ty">Option</span>&lt;<span class="kw-2">&amp;</span>T&gt; {
            <span class="kw">if </span><span class="self">self</span>.state.load(Ordering::Acquire) == LAZY_STATE_DONE {
                <span class="comment">// SAFETY: The DONE state only occurs when data has been fully
                // initialized.
                </span><span class="prelude-val">Some</span>(<span class="kw">unsafe </span>{ <span class="kw-2">&amp;*</span>(<span class="kw-2">*</span><span class="self">self</span>.data.as_ptr()).as_ptr() })
            } <span class="kw">else </span>{
                <span class="prelude-val">None
            </span>}
        }
    }

    <span class="kw">impl</span>&lt;T: fmt::Debug, F: FnMut() -&gt; T&gt; fmt::Debug <span class="kw">for </span>Lazy&lt;T, F&gt; {
        <span class="kw">fn </span>fmt(<span class="kw-2">&amp;</span><span class="self">self</span>, f: <span class="kw-2">&amp;mut </span>fmt::Formatter) -&gt; fmt::Result {
            f.debug_struct(<span class="string">"Lazy"</span>)
                .field(<span class="string">"state"</span>, <span class="kw-2">&amp;</span><span class="self">self</span>.state.load(Ordering::Acquire))
                .field(<span class="string">"create"</span>, <span class="kw-2">&amp;</span><span class="string">"&lt;closure&gt;"</span>)
                .field(<span class="string">"data"</span>, <span class="kw-2">&amp;</span><span class="self">self</span>.poll())
                .finish()
        }
    }

    <span class="kw">impl</span>&lt;T, F&gt; Drop <span class="kw">for </span>Lazy&lt;T, F&gt; {
        <span class="kw">fn </span>drop(<span class="kw-2">&amp;mut </span><span class="self">self</span>) {
            <span class="kw">if </span><span class="kw-2">*</span><span class="self">self</span>.state.get_mut() == LAZY_STATE_DONE {
                <span class="comment">// SAFETY: state is DONE if and only if data has been fully
                // initialized. At which point, it is safe to drop.
                </span><span class="kw">unsafe </span>{
                    <span class="self">self</span>.data.get_mut().assume_init_drop();
                }
            }
        }
    }

    <span class="doccomment">/// A guard that will reset a Lazy's state back to INIT when dropped. The
    /// idea here is to 'forget' this guard on success. On failure (when a
    /// panic occurs), the Drop impl runs and causes all in-progress and future
    /// 'get' calls to panic. Without this guard, all in-progress and future
    /// 'get' calls would spin forever. Crashing is much better than getting
    /// stuck in an infinite loop.
    </span><span class="kw">struct </span>Guard&lt;<span class="lifetime">'a</span>&gt; {
        state: <span class="kw-2">&amp;</span><span class="lifetime">'a </span>AtomicU8,
    }

    <span class="kw">impl</span>&lt;<span class="lifetime">'a</span>&gt; Drop <span class="kw">for </span>Guard&lt;<span class="lifetime">'a</span>&gt; {
        <span class="kw">fn </span>drop(<span class="kw-2">&amp;mut </span><span class="self">self</span>) {
            <span class="comment">// We force ourselves back into an INIT state. This will in turn
            // cause any future 'get' calls to attempt calling 'self.create'
            // again which will in turn panic because 'self.create' will now
            // be 'None'.
            </span><span class="self">self</span>.state.store(LAZY_STATE_INIT, Ordering::Release);
        }
    }
}

<span class="attr">#[cfg(test)]
</span><span class="kw">mod </span>tests {
    <span class="kw">use super</span>::<span class="kw-2">*</span>;

    <span class="kw">fn </span>assert_send&lt;T: Send&gt;() {}
    <span class="kw">fn </span>assert_sync&lt;T: Sync&gt;() {}
    <span class="kw">fn </span>assert_unwind&lt;T: core::panic::UnwindSafe&gt;() {}
    <span class="kw">fn </span>assert_refunwind&lt;T: core::panic::RefUnwindSafe&gt;() {}

    <span class="attr">#[test]
    </span><span class="kw">fn </span>oibits() {
        assert_send::&lt;Lazy&lt;u64&gt;&gt;();
        assert_sync::&lt;Lazy&lt;u64&gt;&gt;();
        assert_unwind::&lt;Lazy&lt;u64&gt;&gt;();
        assert_refunwind::&lt;Lazy&lt;u64&gt;&gt;();
    }

    <span class="comment">// This is a regression test because we used to rely on the inferred Sync
    // impl for the Lazy type defined above (for 'alloc' mode). In the
    // inferred impl, it only requires that T: Sync for Lazy&lt;T&gt;: Sync. But
    // if we have that, we can actually make use of the fact that Lazy&lt;T&gt; drops
    // T to create a value on one thread and drop it on another. This *should*
    // require T: Send, but our missing bounds before let it sneak by.
    //
    // Basically, this test should not compile, so we... comment it out. We
    // don't have a great way of testing compile-fail tests right now.
    //
    // See: https://github.com/BurntSushi/regex-automata/issues/30
    /*
    #[test]
    fn sync_not_send() {
        #[allow(dead_code)]
        fn inner&lt;T: Sync + Default&gt;() {
            let lazy = Lazy::new(move || T::default());
            std::thread::scope(|scope| {
                scope.spawn(|| {
                    Lazy::get(&amp;lazy); // We create T in this thread
                });
            });
            // And drop in this thread.
            drop(lazy);
            // So we have send a !Send type over threads. (with some more
            // legwork, its possible to even sneak the value out of drop
            // through thread local)
        }
    }
    */
</span>}
</code></pre></div></section></main></body></html>