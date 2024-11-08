<!DOCTYPE html><html lang="en"><head><meta charset="utf-8"><meta name="viewport" content="width=device-width, initial-scale=1.0"><meta name="generator" content="rustdoc"><meta name="description" content="Source of the Rust file `/Users/jeyakumar/.cargo/registry/src/index.crates.io-6f17d22bba15001f/aho-corasick-1.1.3/src/util/byte_frequencies.rs`."><title>byte_frequencies.rs - source</title><script>if(window.location.protocol!=="file:")document.head.insertAdjacentHTML("beforeend","SourceSerif4-Regular-46f98efaafac5295.ttf.woff2,FiraSans-Regular-018c141bf0843ffd.woff2,FiraSans-Medium-8f9a781e4970d388.woff2,SourceCodePro-Regular-562dcc5011b6de7d.ttf.woff2,SourceCodePro-Semibold-d899c5a5c4aeb14a.ttf.woff2".split(",").map(f=>`<link rel="preload" as="font" type="font/woff2" crossorigin href="../../../static.files/${f}">`).join(""))</script><link rel="stylesheet" href="../../../static.files/normalize-76eba96aa4d2e634.css"><link rel="stylesheet" href="../../../static.files/rustdoc-081576b923113409.css"><meta name="rustdoc-vars" data-root-path="../../../" data-static-root-path="../../../static.files/" data-current-crate="aho_corasick" data-themes="" data-resource-suffix="" data-rustdoc-version="1.79.0 (129f3b996 2024-06-10)" data-channel="1.79.0" data-search-js="search-bf21c90c8c1d92b1.js" data-settings-js="settings-4313503d2e1961c2.js" ><script src="../../../static.files/storage-e32f0c247825364d.js"></script><script defer src="../../../static.files/src-script-e66d777a5a92e9b2.js"></script><script defer src="../../../src-files.js"></script><script defer src="../../../static.files/main-20a3ad099b048cf2.js"></script><noscript><link rel="stylesheet" href="../../../static.files/noscript-09095024cf37855e.css"></noscript><link rel="alternate icon" type="image/png" href="../../../static.files/favicon-32x32-422f7d1d52889060.png"><link rel="icon" type="image/svg+xml" href="../../../static.files/favicon-2c020d218678b618.svg"></head><body class="rustdoc src"><!--[if lte IE 11]><div class="warning">This old browser is unsupported and will most likely display funky things.</div><![endif]--><nav class="sidebar"><div class="src-sidebar-title"><h2>Files</h2></div></nav><div class="sidebar-resizer"></div><main><nav class="sub"><form class="search-form"><span></span><div id="sidebar-button" tabindex="-1"><a href="../../../aho_corasick/all.html" title="show sidebar"></a></div><input class="search-input" name="search" aria-label="Run search in the documentation" autocomplete="off" spellcheck="false" placeholder="Type ‘S’ or ‘/’ to search, ‘?’ for more options…" type="search"><div id="help-button" tabindex="-1"><a href="../../../help.html" title="help">?</a></div><div id="settings-menu" tabindex="-1"><a href="../../../settings.html" title="settings">Settings</a></div></form></nav><section id="main-content" class="content"><div class="example-wrap"><div data-nosnippet><pre class="src-line-numbers"><a href="#1" id="1">1</a>
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
</pre></div><pre class="rust"><code><span class="kw">pub const </span>BYTE_FREQUENCIES: [u8; <span class="number">256</span>] = [
    <span class="number">55</span>,  <span class="comment">// '\x00'
    </span><span class="number">52</span>,  <span class="comment">// '\x01'
    </span><span class="number">51</span>,  <span class="comment">// '\x02'
    </span><span class="number">50</span>,  <span class="comment">// '\x03'
    </span><span class="number">49</span>,  <span class="comment">// '\x04'
    </span><span class="number">48</span>,  <span class="comment">// '\x05'
    </span><span class="number">47</span>,  <span class="comment">// '\x06'
    </span><span class="number">46</span>,  <span class="comment">// '\x07'
    </span><span class="number">45</span>,  <span class="comment">// '\x08'
    </span><span class="number">103</span>, <span class="comment">// '\t'
    </span><span class="number">242</span>, <span class="comment">// '\n'
    </span><span class="number">66</span>,  <span class="comment">// '\x0b'
    </span><span class="number">67</span>,  <span class="comment">// '\x0c'
    </span><span class="number">229</span>, <span class="comment">// '\r'
    </span><span class="number">44</span>,  <span class="comment">// '\x0e'
    </span><span class="number">43</span>,  <span class="comment">// '\x0f'
    </span><span class="number">42</span>,  <span class="comment">// '\x10'
    </span><span class="number">41</span>,  <span class="comment">// '\x11'
    </span><span class="number">40</span>,  <span class="comment">// '\x12'
    </span><span class="number">39</span>,  <span class="comment">// '\x13'
    </span><span class="number">38</span>,  <span class="comment">// '\x14'
    </span><span class="number">37</span>,  <span class="comment">// '\x15'
    </span><span class="number">36</span>,  <span class="comment">// '\x16'
    </span><span class="number">35</span>,  <span class="comment">// '\x17'
    </span><span class="number">34</span>,  <span class="comment">// '\x18'
    </span><span class="number">33</span>,  <span class="comment">// '\x19'
    </span><span class="number">56</span>,  <span class="comment">// '\x1a'
    </span><span class="number">32</span>,  <span class="comment">// '\x1b'
    </span><span class="number">31</span>,  <span class="comment">// '\x1c'
    </span><span class="number">30</span>,  <span class="comment">// '\x1d'
    </span><span class="number">29</span>,  <span class="comment">// '\x1e'
    </span><span class="number">28</span>,  <span class="comment">// '\x1f'
    </span><span class="number">255</span>, <span class="comment">// ' '
    </span><span class="number">148</span>, <span class="comment">// '!'
    </span><span class="number">164</span>, <span class="comment">// '"'
    </span><span class="number">149</span>, <span class="comment">// '#'
    </span><span class="number">136</span>, <span class="comment">// '$'
    </span><span class="number">160</span>, <span class="comment">// '%'
    </span><span class="number">155</span>, <span class="comment">// '&amp;'
    </span><span class="number">173</span>, <span class="comment">// "'"
    </span><span class="number">221</span>, <span class="comment">// '('
    </span><span class="number">222</span>, <span class="comment">// ')'
    </span><span class="number">134</span>, <span class="comment">// '*'
    </span><span class="number">122</span>, <span class="comment">// '+'
    </span><span class="number">232</span>, <span class="comment">// ','
    </span><span class="number">202</span>, <span class="comment">// '-'
    </span><span class="number">215</span>, <span class="comment">// '.'
    </span><span class="number">224</span>, <span class="comment">// '/'
    </span><span class="number">208</span>, <span class="comment">// '0'
    </span><span class="number">220</span>, <span class="comment">// '1'
    </span><span class="number">204</span>, <span class="comment">// '2'
    </span><span class="number">187</span>, <span class="comment">// '3'
    </span><span class="number">183</span>, <span class="comment">// '4'
    </span><span class="number">179</span>, <span class="comment">// '5'
    </span><span class="number">177</span>, <span class="comment">// '6'
    </span><span class="number">168</span>, <span class="comment">// '7'
    </span><span class="number">178</span>, <span class="comment">// '8'
    </span><span class="number">200</span>, <span class="comment">// '9'
    </span><span class="number">226</span>, <span class="comment">// ':'
    </span><span class="number">195</span>, <span class="comment">// ';'
    </span><span class="number">154</span>, <span class="comment">// '&lt;'
    </span><span class="number">184</span>, <span class="comment">// '='
    </span><span class="number">174</span>, <span class="comment">// '&gt;'
    </span><span class="number">126</span>, <span class="comment">// '?'
    </span><span class="number">120</span>, <span class="comment">// '@'
    </span><span class="number">191</span>, <span class="comment">// 'A'
    </span><span class="number">157</span>, <span class="comment">// 'B'
    </span><span class="number">194</span>, <span class="comment">// 'C'
    </span><span class="number">170</span>, <span class="comment">// 'D'
    </span><span class="number">189</span>, <span class="comment">// 'E'
    </span><span class="number">162</span>, <span class="comment">// 'F'
    </span><span class="number">161</span>, <span class="comment">// 'G'
    </span><span class="number">150</span>, <span class="comment">// 'H'
    </span><span class="number">193</span>, <span class="comment">// 'I'
    </span><span class="number">142</span>, <span class="comment">// 'J'
    </span><span class="number">137</span>, <span class="comment">// 'K'
    </span><span class="number">171</span>, <span class="comment">// 'L'
    </span><span class="number">176</span>, <span class="comment">// 'M'
    </span><span class="number">185</span>, <span class="comment">// 'N'
    </span><span class="number">167</span>, <span class="comment">// 'O'
    </span><span class="number">186</span>, <span class="comment">// 'P'
    </span><span class="number">112</span>, <span class="comment">// 'Q'
    </span><span class="number">175</span>, <span class="comment">// 'R'
    </span><span class="number">192</span>, <span class="comment">// 'S'
    </span><span class="number">188</span>, <span class="comment">// 'T'
    </span><span class="number">156</span>, <span class="comment">// 'U'
    </span><span class="number">140</span>, <span class="comment">// 'V'
    </span><span class="number">143</span>, <span class="comment">// 'W'
    </span><span class="number">123</span>, <span class="comment">// 'X'
    </span><span class="number">133</span>, <span class="comment">// 'Y'
    </span><span class="number">128</span>, <span class="comment">// 'Z'
    </span><span class="number">147</span>, <span class="comment">// '['
    </span><span class="number">138</span>, <span class="comment">// '\\'
    </span><span class="number">146</span>, <span class="comment">// ']'
    </span><span class="number">114</span>, <span class="comment">// '^'
    </span><span class="number">223</span>, <span class="comment">// '_'
    </span><span class="number">151</span>, <span class="comment">// '`'
    </span><span class="number">249</span>, <span class="comment">// 'a'
    </span><span class="number">216</span>, <span class="comment">// 'b'
    </span><span class="number">238</span>, <span class="comment">// 'c'
    </span><span class="number">236</span>, <span class="comment">// 'd'
    </span><span class="number">253</span>, <span class="comment">// 'e'
    </span><span class="number">227</span>, <span class="comment">// 'f'
    </span><span class="number">218</span>, <span class="comment">// 'g'
    </span><span class="number">230</span>, <span class="comment">// 'h'
    </span><span class="number">247</span>, <span class="comment">// 'i'
    </span><span class="number">135</span>, <span class="comment">// 'j'
    </span><span class="number">180</span>, <span class="comment">// 'k'
    </span><span class="number">241</span>, <span class="comment">// 'l'
    </span><span class="number">233</span>, <span class="comment">// 'm'
    </span><span class="number">246</span>, <span class="comment">// 'n'
    </span><span class="number">244</span>, <span class="comment">// 'o'
    </span><span class="number">231</span>, <span class="comment">// 'p'
    </span><span class="number">139</span>, <span class="comment">// 'q'
    </span><span class="number">245</span>, <span class="comment">// 'r'
    </span><span class="number">243</span>, <span class="comment">// 's'
    </span><span class="number">251</span>, <span class="comment">// 't'
    </span><span class="number">235</span>, <span class="comment">// 'u'
    </span><span class="number">201</span>, <span class="comment">// 'v'
    </span><span class="number">196</span>, <span class="comment">// 'w'
    </span><span class="number">240</span>, <span class="comment">// 'x'
    </span><span class="number">214</span>, <span class="comment">// 'y'
    </span><span class="number">152</span>, <span class="comment">// 'z'
    </span><span class="number">182</span>, <span class="comment">// '{'
    </span><span class="number">205</span>, <span class="comment">// '|'
    </span><span class="number">181</span>, <span class="comment">// '}'
    </span><span class="number">127</span>, <span class="comment">// '~'
    </span><span class="number">27</span>,  <span class="comment">// '\x7f'
    </span><span class="number">212</span>, <span class="comment">// '\x80'
    </span><span class="number">211</span>, <span class="comment">// '\x81'
    </span><span class="number">210</span>, <span class="comment">// '\x82'
    </span><span class="number">213</span>, <span class="comment">// '\x83'
    </span><span class="number">228</span>, <span class="comment">// '\x84'
    </span><span class="number">197</span>, <span class="comment">// '\x85'
    </span><span class="number">169</span>, <span class="comment">// '\x86'
    </span><span class="number">159</span>, <span class="comment">// '\x87'
    </span><span class="number">131</span>, <span class="comment">// '\x88'
    </span><span class="number">172</span>, <span class="comment">// '\x89'
    </span><span class="number">105</span>, <span class="comment">// '\x8a'
    </span><span class="number">80</span>,  <span class="comment">// '\x8b'
    </span><span class="number">98</span>,  <span class="comment">// '\x8c'
    </span><span class="number">96</span>,  <span class="comment">// '\x8d'
    </span><span class="number">97</span>,  <span class="comment">// '\x8e'
    </span><span class="number">81</span>,  <span class="comment">// '\x8f'
    </span><span class="number">207</span>, <span class="comment">// '\x90'
    </span><span class="number">145</span>, <span class="comment">// '\x91'
    </span><span class="number">116</span>, <span class="comment">// '\x92'
    </span><span class="number">115</span>, <span class="comment">// '\x93'
    </span><span class="number">144</span>, <span class="comment">// '\x94'
    </span><span class="number">130</span>, <span class="comment">// '\x95'
    </span><span class="number">153</span>, <span class="comment">// '\x96'
    </span><span class="number">121</span>, <span class="comment">// '\x97'
    </span><span class="number">107</span>, <span class="comment">// '\x98'
    </span><span class="number">132</span>, <span class="comment">// '\x99'
    </span><span class="number">109</span>, <span class="comment">// '\x9a'
    </span><span class="number">110</span>, <span class="comment">// '\x9b'
    </span><span class="number">124</span>, <span class="comment">// '\x9c'
    </span><span class="number">111</span>, <span class="comment">// '\x9d'
    </span><span class="number">82</span>,  <span class="comment">// '\x9e'
    </span><span class="number">108</span>, <span class="comment">// '\x9f'
    </span><span class="number">118</span>, <span class="comment">// '\xa0'
    </span><span class="number">141</span>, <span class="comment">// '¡'
    </span><span class="number">113</span>, <span class="comment">// '¢'
    </span><span class="number">129</span>, <span class="comment">// '£'
    </span><span class="number">119</span>, <span class="comment">// '¤'
    </span><span class="number">125</span>, <span class="comment">// '¥'
    </span><span class="number">165</span>, <span class="comment">// '¦'
    </span><span class="number">117</span>, <span class="comment">// '§'
    </span><span class="number">92</span>,  <span class="comment">// '¨'
    </span><span class="number">106</span>, <span class="comment">// '©'
    </span><span class="number">83</span>,  <span class="comment">// 'ª'
    </span><span class="number">72</span>,  <span class="comment">// '«'
    </span><span class="number">99</span>,  <span class="comment">// '¬'
    </span><span class="number">93</span>,  <span class="comment">// '\xad'
    </span><span class="number">65</span>,  <span class="comment">// '®'
    </span><span class="number">79</span>,  <span class="comment">// '¯'
    </span><span class="number">166</span>, <span class="comment">// '°'
    </span><span class="number">237</span>, <span class="comment">// '±'
    </span><span class="number">163</span>, <span class="comment">// '²'
    </span><span class="number">199</span>, <span class="comment">// '³'
    </span><span class="number">190</span>, <span class="comment">// '´'
    </span><span class="number">225</span>, <span class="comment">// 'µ'
    </span><span class="number">209</span>, <span class="comment">// '¶'
    </span><span class="number">203</span>, <span class="comment">// '·'
    </span><span class="number">198</span>, <span class="comment">// '¸'
    </span><span class="number">217</span>, <span class="comment">// '¹'
    </span><span class="number">219</span>, <span class="comment">// 'º'
    </span><span class="number">206</span>, <span class="comment">// '»'
    </span><span class="number">234</span>, <span class="comment">// '¼'
    </span><span class="number">248</span>, <span class="comment">// '½'
    </span><span class="number">158</span>, <span class="comment">// '¾'
    </span><span class="number">239</span>, <span class="comment">// '¿'
    </span><span class="number">255</span>, <span class="comment">// 'À'
    </span><span class="number">255</span>, <span class="comment">// 'Á'
    </span><span class="number">255</span>, <span class="comment">// 'Â'
    </span><span class="number">255</span>, <span class="comment">// 'Ã'
    </span><span class="number">255</span>, <span class="comment">// 'Ä'
    </span><span class="number">255</span>, <span class="comment">// 'Å'
    </span><span class="number">255</span>, <span class="comment">// 'Æ'
    </span><span class="number">255</span>, <span class="comment">// 'Ç'
    </span><span class="number">255</span>, <span class="comment">// 'È'
    </span><span class="number">255</span>, <span class="comment">// 'É'
    </span><span class="number">255</span>, <span class="comment">// 'Ê'
    </span><span class="number">255</span>, <span class="comment">// 'Ë'
    </span><span class="number">255</span>, <span class="comment">// 'Ì'
    </span><span class="number">255</span>, <span class="comment">// 'Í'
    </span><span class="number">255</span>, <span class="comment">// 'Î'
    </span><span class="number">255</span>, <span class="comment">// 'Ï'
    </span><span class="number">255</span>, <span class="comment">// 'Ð'
    </span><span class="number">255</span>, <span class="comment">// 'Ñ'
    </span><span class="number">255</span>, <span class="comment">// 'Ò'
    </span><span class="number">255</span>, <span class="comment">// 'Ó'
    </span><span class="number">255</span>, <span class="comment">// 'Ô'
    </span><span class="number">255</span>, <span class="comment">// 'Õ'
    </span><span class="number">255</span>, <span class="comment">// 'Ö'
    </span><span class="number">255</span>, <span class="comment">// '×'
    </span><span class="number">255</span>, <span class="comment">// 'Ø'
    </span><span class="number">255</span>, <span class="comment">// 'Ù'
    </span><span class="number">255</span>, <span class="comment">// 'Ú'
    </span><span class="number">255</span>, <span class="comment">// 'Û'
    </span><span class="number">255</span>, <span class="comment">// 'Ü'
    </span><span class="number">255</span>, <span class="comment">// 'Ý'
    </span><span class="number">255</span>, <span class="comment">// 'Þ'
    </span><span class="number">255</span>, <span class="comment">// 'ß'
    </span><span class="number">255</span>, <span class="comment">// 'à'
    </span><span class="number">255</span>, <span class="comment">// 'á'
    </span><span class="number">255</span>, <span class="comment">// 'â'
    </span><span class="number">255</span>, <span class="comment">// 'ã'
    </span><span class="number">255</span>, <span class="comment">// 'ä'
    </span><span class="number">255</span>, <span class="comment">// 'å'
    </span><span class="number">255</span>, <span class="comment">// 'æ'
    </span><span class="number">255</span>, <span class="comment">// 'ç'
    </span><span class="number">255</span>, <span class="comment">// 'è'
    </span><span class="number">255</span>, <span class="comment">// 'é'
    </span><span class="number">255</span>, <span class="comment">// 'ê'
    </span><span class="number">255</span>, <span class="comment">// 'ë'
    </span><span class="number">255</span>, <span class="comment">// 'ì'
    </span><span class="number">255</span>, <span class="comment">// 'í'
    </span><span class="number">255</span>, <span class="comment">// 'î'
    </span><span class="number">255</span>, <span class="comment">// 'ï'
    </span><span class="number">255</span>, <span class="comment">// 'ð'
    </span><span class="number">255</span>, <span class="comment">// 'ñ'
    </span><span class="number">255</span>, <span class="comment">// 'ò'
    </span><span class="number">255</span>, <span class="comment">// 'ó'
    </span><span class="number">255</span>, <span class="comment">// 'ô'
    </span><span class="number">255</span>, <span class="comment">// 'õ'
    </span><span class="number">255</span>, <span class="comment">// 'ö'
    </span><span class="number">255</span>, <span class="comment">// '÷'
    </span><span class="number">255</span>, <span class="comment">// 'ø'
    </span><span class="number">255</span>, <span class="comment">// 'ù'
    </span><span class="number">255</span>, <span class="comment">// 'ú'
    </span><span class="number">255</span>, <span class="comment">// 'û'
    </span><span class="number">255</span>, <span class="comment">// 'ü'
    </span><span class="number">255</span>, <span class="comment">// 'ý'
    </span><span class="number">255</span>, <span class="comment">// 'þ'
    </span><span class="number">255</span>, <span class="comment">// 'ÿ'
</span>];
</code></pre></div></section></main></body></html>