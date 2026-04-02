# HTTP/3 连接测试失败报告

## 报告概要

- **生成时间**: 2025/12/10 18:44:13
- **数据来源**: connectivity_results-20251210-184413.json
- **总测试数**: 483
- **失败测试数**: 11
- **成功测试数**: 472
- **失败率**: 2.28%
- **平均延迟**: 91.76ms
- **最小延迟**: 47ms
- **最大延迟**: 879ms

---

## 失败测试详情

### 📊 错误类型统计

- **连接超时: I/O超时**: 11 次 (100.0%)

### 🔍 按错误类型分类的失败测试详情

#### 连接超时: I/O超时 (11 次测试)

| 序号 | 主机/域名                  | 目标IP          | IP版本 | 协议 | 状态码 | 延迟(ms) | 服务器 | 错误信息                                  |
| ---- | -------------------------- | --------------- | ------ | ---- | ------ | -------- | ------ | ----------------------------------------- |
| 7    | 115.22.115.218             | 115.22.115.218  | IPv4   | none | N/A    | 0        | N/A    | dial tcp 115.22.115.218:443: i/o timeout  |
| 57   | 52.76.110.129              | 52.76.110.129   | IPv4   | none | N/A    | 0        | N/A    | dial tcp 52.76.110.129:443: i/o timeout   |
| 80   | 121.188.182.190            | 121.188.182.190 | IPv4   | none | N/A    | 0        | N/A    | dial tcp 121.188.182.190:443: i/o timeout |
| 190  | cfip.xxxxxxxx.tk           | 198.41.212.130  | IPv4   | none | N/A    | 0        | N/A    | dial tcp 198.41.212.130:443: i/o timeout  |
| 214  | 111.171.108.67             | 111.171.108.67  | IPv4   | none | N/A    | 0        | N/A    | dial tcp 111.171.108.67:443: i/o timeout  |
| 245  | 175.212.207.13             | 175.212.207.13  | IPv4   | none | N/A    | 0        | N/A    | dial tcp 175.212.207.13:443: i/o timeout  |
| 332  | sullivan.ns.cloudflare.com | 162.159.44.161  | IPv4   | none | N/A    | 0        | N/A    | dial tcp 162.159.44.161:443: i/o timeout  |
| 383  | benedict.ns.cloudflare.com | 108.162.195.205 | IPv4   | none | N/A    | 0        | N/A    | dial tcp 108.162.195.205:443: i/o timeout |
| 385  | benedict.ns.cloudflare.com | 172.64.35.205   | IPv4   | none | N/A    | 0        | N/A    | dial tcp 172.64.35.205:443: i/o timeout   |
| 449  | ct.877774.xyz              | ct.877774.xyz   | IPv4   | none | N/A    | 0        | N/A    | dial tcp ct.877774.xyz:443: i/o timeout   |
| 450  | 119.194.220.146            | 119.194.220.146 | IPv4   | none | N/A    | 0        | N/A    | dial tcp 119.194.220.146:443: i/o timeout |

### 📈 错误分析总结

#### 主错误类型分布

- **连接超时**: 11 次 (100.0%)

#### 错误模式分析

**超时集中度分析**: 共有 11 次超时，主要集中在IP段 172.64（2
次），可能存在网络路由问题或目标服务器负载过高

**协议协商分析**: 有 11 次失败是因为协议协商失败（protocol:
none），说明无法与目标建立HTTP/3或其他现代协议连接

**IP版本分析**:
所有失败的测试都使用IPv4，IPv6连接可能更稳定或目标服务器的IPv6配置更好

---

## 🚀 延迟最低的 100 条记录

以下显示了延迟最低的测试记录，包括成功和失败的测试：

| 序号 | 主机/域名 | 目标IP | IP版本 | 协议 | 状态 | 延迟(ms) | 服务器 |
|------|-----------|--------|--------|------|------|----------|--------|| 125 |
steamdb.info | 104.20.34.212 | IPv4 | h2 | ✅ 成功 | 47 | cloudflare | | 295 |
time.is | 2606:4700:20::681a:c36 | IPv6 | h2 | ✅ 成功 | 48 | cloudflare | | 88
| saas.sin.fan | 162.159.36.5 | IPv4 | h2 | ✅ 成功 | 49 | cloudflare | | 212 |
yx-auto.pages.dev | 2606:4700:310c::ac42:2f70 | IPv6 | h2 | ✅ 成功 | 49 |
cloudflare | | 249 | xn--b6gac.eu.org | 2606:4700:3037::ac43:99fd | IPv6 | h2 |
✅ 成功 | 50 | cloudflare | | 299 | ipv4.ip.sb | 172.67.75.172 | IPv4 | h2 | ✅
成功 | 52 | cloudflare | | 325 | singapore.com | 104.26.12.140 | IPv4 | h2 | ✅
成功 | 52 | cloudflare | | 342 | toy-people.com | 2606:4700:20::681a:324 | IPv6
| h2 | ✅ 成功 | 52 | cloudflare | | 440 | trevor.ns.cloudflare.com |
2606:4700:58::a29f:2c9a | IPv6 | h2 | ✅ 成功 | 52 | cloudflare | | 136 |
ct.877774.xyz | 172.64.229.217 | IPv4 | h2 | ✅ 成功 | 53 | cloudflare | | 165 |
yx-auto.pages.dev | 2606:4700:3031::ac43:868b | IPv6 | h2 | ✅ 成功 | 53 |
cloudflare | | 230 | 4444.cloudflare.182682.xyz | 172.67.152.183 | IPv4 | h2 | ✅ 成功 | 53 |
cloudflare | | 246 | 104.17.142.12 | 104.17.142.12 | IPv4 | h2 | ✅ 成功 | 53 |
cloudflare | | 250 | xn--b6gac.eu.org | 2606:4700:3035::6815:5a4e | IPv6 | h2 |
✅ 成功 | 53 | cloudflare | | 425 | palera.in | 2606:4700:3032::ac43:9d7a | IPv6
| h2 | ✅ 成功 | 53 | cloudflare | | 121 | ae8a9c24-83de.masx200.ddns-ip.net |
172.67.157.182 | IPv4 | h2 | ✅ 成功 | 54 | cloudflare | | 223 |
local-aria2-webui.masx200.ddns-ip.net | 104.21.14.41 | IPv4 | h2 | ✅ 成功 | 54
| cloudflare | | 298 | ipv4.ip.sb | 104.26.12.31 | IPv4 | h2 | ✅ 成功 | 54 |
cloudflare | | 84 | tasteatlas.com | 2606:4700::6811:2569 | IPv6 | h2 | ✅ 成功
| 55 | cloudflare | | 202 | japan.com | 172.67.70.92 | IPv4 | h2 | ✅ 成功 | 55
| cloudflare | | 271 | [2606:4700:440f::53aa:4126] | 2606:4700:440f::53aa:4126 |
IPv6 | h2 | ✅ 成功 | 55 | cloudflare | | 303 | fbi.gov | 2606:4700::6810:94f4 |
IPv6 | h2 | ✅ 成功 | 55 | cloudflare | | 309 | shopify.com | 23.227.38.33 |
IPv4 | h2 | ✅ 成功 | 55 | cloudflare | | 352 | cmcc.877774.xyz | 104.16.148.244
| IPv4 | h2 | ✅ 成功 | 55 | cloudflare | | 359 | cmcc.877774.xyz | 104.16.149.7
| IPv4 | h2 | ✅ 成功 | 55 | cloudflare | | 447 | wilson.ns.cloudflare.com |
2803:f800:50::6ca2:c36e | IPv6 | h2 | ✅ 成功 | 55 | cloudflare | | 128 |
steamdb.info | 2606:4700:10::ac42:affa | IPv6 | h2 | ✅ 成功 | 56 | cloudflare |
| 192 | cfip.xxxxxxxx.tk | 104.27.21.118 | IPv4 | h2 | ✅ 成功 | 56 | cloudflare
| | 198 | www.whatismyip.com | 2606:4700:20::ac43:4581 | IPv6 | h2 | ✅ 成功 |
56 | cloudflare | | 272 | 172.64.153.172 | 172.64.153.172 | IPv4 | h2 | ✅ 成功
| 56 | cloudflare | | 293 | time.is | 104.26.13.54 | IPv4 | h2 | ✅ 成功 | 56 |
cloudflare | | 294 | time.is | 2606:4700:20::681a:d36 | IPv6 | h2 | ✅ 成功 | 56
| cloudflare | | 306 | 172.67.79.211 | 172.67.79.211 | IPv4 | h2 | ✅ 成功 | 56
| cloudflare | | 329 | singapore.com | 2606:4700:20::ac43:4bc2 | IPv6 | h2 | ✅
成功 | 56 | cloudflare | | 347 | cmcc.877774.xyz | 104.16.148.8 | IPv4 | h2 | ✅
成功 | 56 | cloudflare | | 396 | 172.67.106.26 | 172.67.106.26 | IPv4 | h2 | ✅
成功 | 56 | cloudflare | | 427 | www.okcupid.com | 104.18.160.63 | IPv4 | h2 |
✅ 成功 | 56 | cloudflare | | 161 | lewis.ns.cloudflare.com |
2a06:98c1:50::ac40:239f | IPv6 | h2 | ✅ 成功 | 57 | cloudflare | | 248 |
xn--b6gac.eu.org | 104.21.90.78 | IPv4 | h2 | ✅ 成功 | 57 | cloudflare | | 257
| ip.sb | 2606:4700:20::ac43:4bac | IPv6 | h2 | ✅ 成功 | 57 | cloudflare | |
341 | toy-people.com | 2606:4700:20::681a:224 | IPv6 | h2 | ✅ 成功 | 57 |
cloudflare | | 365 | cmcc.877774.xyz | 104.16.149.244 | IPv4 | h2 | ✅ 成功 | 57
| cloudflare | | 389 | uriah.ns.cloudflare.com | 108.162.195.194 | IPv4 | h2 |
✅ 成功 | 57 | cloudflare | | 400 | moura.ns.cloudflare.com |
2606:4700:58::a29f:2cd9 | IPv6 | h2 | ✅ 成功 | 57 | cloudflare | | 428 |
www.okcupid.com | 104.16.144.63 | IPv4 | h2 | ✅ 成功 | 57 | cloudflare | | 475
| icook.tw | 104.20.28.74 | IPv4 | h2 | ✅ 成功 | 57 | cloudflare | | 113 |
cf.0sm.com | 2606:4700:3032::6815:785 | IPv6 | h2 | ✅ 成功 | 58 | cloudflare |
| 134 | ct.877774.xyz | 172.64.229.185 | IPv4 | h2 | ✅ 成功 | 58 | cloudflare |
| 194 | 104.16.45.84 | 104.16.45.84 | IPv4 | h2 | ✅ 成功 | 58 | cloudflare | |
237 | craig.ns.cloudflare.com | 2606:4700:58::a29f:2cc0 | IPv6 | h2 | ✅ 成功 |
58 | cloudflare | | 252 | ip.sb | 104.26.13.31 | IPv4 | h2 | ✅ 成功 | 58 |
cloudflare | | 348 | cmcc.877774.xyz | 104.16.148.9 | IPv4 | h2 | ✅ 成功 | 58 |
cloudflare | | 353 | cmcc.877774.xyz | 104.16.149.1 | IPv4 | h2 | ✅ 成功 | 58 |
cloudflare | | 446 | wilson.ns.cloudflare.com | 2606:4700:58::a29f:2c6e | IPv6 |
h2 | ✅ 成功 | 58 | cloudflare | | 120 | na.877774.xyz | 104.18.187.25 | IPv4 |
h2 | ✅ 成功 | 59 | cloudflare | | 176 | www.digitalocean.com |
2606:4700::6813:ad44 | IPv6 | h2 | ✅ 成功 | 59 | cloudflare | | 195 |
www.whatismyip.com | 104.26.12.23 | IPv4 | h2 | ✅ 成功 | 59 | cloudflare | |
200 | www.whatismyip.com | 2606:4700:20::681a:d17 | IPv6 | h2 | ✅ 成功 | 59 |
cloudflare | | 209 | yx-auto.pages.dev | 172.66.44.144 | IPv4 | h2 | ✅ 成功 |
59 | cloudflare | | 211 | yx-auto.pages.dev | 2606:4700:310c::ac42:2c90 | IPv6 |
h2 | ✅ 成功 | 59 | cloudflare | | 231 | 4444.cloudflare.182682.xyz | 104.21.80.180 | IPv4 |
h2 | ✅ 成功 | 59 | cloudflare | | 233 | 4444.cloudflare.182682.xyz |
2606:4700:3033::6815:50b4 | IPv6 | h2 | ✅ 成功 | 59 | cloudflare | | 286 |
cu.877774.xyz | 104.26.4.118 | IPv4 | h2 | ✅ 成功 | 59 | cloudflare | | 326 |
singapore.com | 172.67.75.194 | IPv4 | h2 | ✅ 成功 | 59 | cloudflare | | 379 |
www.visa.com.hk | 104.18.21.69 | IPv4 | h2 | ✅ 成功 | 59 | cloudflare | | 421 |
162.159.133.85 | 162.159.133.85 | IPv4 | h2 | ✅ 成功 | 59 | cloudflare | | 95 |
zread.ai | 2606:4700:3032::ac43:ca4e | IPv6 | h2 | ✅ 成功 | 60 | cloudflare | |
98 | ipinfo.in | 104.21.21.129 | IPv4 | h2 | ✅ 成功 | 60 | cloudflare | | 104 |
cf.zhetengsha.eu.org | 2606:4700:4407::ac40:9052 | IPv6 | h2 | ✅ 成功 | 60 |
cloudflare | | 106 | 172.64.154.18 | 172.64.154.18 | IPv4 | h2 | ✅ 成功 | 60 |
cloudflare | | 186 | comicabc.com | 2606:4700:3036::6815:400a | IPv6 | h2 | ✅
成功 | 60 | cloudflare | | 232 | 4444.cloudflare.182682.xyz | 2606:4700:3033::ac43:98b7 |
IPv6 | h2 | ✅ 成功 | 60 | cloudflare | | 242 | www.ipget.net |
2606:4700:3036::6815:fd4 | IPv6 | h2 | ✅ 成功 | 60 | cloudflare | | 262 |
braden.ns.cloudflare.com | 2803:f800:50::6ca2:c3a9 | IPv6 | h2 | ✅ 成功 | 60 |
cloudflare | | 278 | abdullah.ns.cloudflare.com | 2a06:98c1:50::ac40:23cb | IPv6
| h2 | ✅ 成功 | 60 | cloudflare | | 285 | cu.877774.xyz | 104.26.4.117 | IPv4 |
h2 | ✅ 成功 | 60 | cloudflare | | 340 | toy-people.com |
2606:4700:20::ac43:4812 | IPv6 | h2 | ✅ 成功 | 60 | cloudflare | | 355 |
cmcc.877774.xyz | 104.16.149.3 | IPv4 | h2 | ✅ 成功 | 60 | cloudflare | | 164 |
yx-auto.pages.dev | 2606:4700:3030::6815:63c | IPv6 | h2 | ✅ 成功 | 61 |
cloudflare | | 225 | local-aria2-webui.masx200.ddns-ip.net |
2606:4700:3031::ac43:9db6 | IPv6 | h2 | ✅ 成功 | 61 | cloudflare | | 255 |
ip.sb | 2606:4700:20::681a:c1f | IPv6 | h2 | ✅ 成功 | 61 | cloudflare | | 287 |
cu.877774.xyz | 104.26.4.119 | IPv4 | h2 | ✅ 成功 | 61 | cloudflare | | 360 |
cmcc.877774.xyz | 104.16.149.8 | IPv4 | h2 | ✅ 成功 | 61 | cloudflare | | 371 |
172.64.144.188 | 172.64.144.188 | IPv4 | h2 | ✅ 成功 | 61 | cloudflare | | 378
| www.visa.com.hk | 104.18.20.69 | IPv4 | h2 | ✅ 成功 | 61 | cloudflare | | 419
| rustam.ns.cloudflare.com | 2803:f800:50::6ca2:c394 | IPv6 | h2 | ✅ 成功 | 61
| cloudflare | | 429 | www.okcupid.com | 104.17.48.63 | IPv4 | h2 | ✅ 成功 | 61
| cloudflare | | 434 | pranab.ns.cloudflare.com | 2606:4700:58::a29f:2cc7 | IPv6
| h2 | ✅ 成功 | 61 | cloudflare | | 452 | www.4444.cloudflare.182682.xyz | 162.159.152.2 | IPv4 |
h2 | ✅ 成功 | 61 | cloudflare | | 483 | damien.ns.cloudflare.com |
2a06:98c1:50::ac40:23a8 | IPv6 | h2 | ✅ 成功 | 61 | cloudflare | | 77 |
cloudflare-ip.mofashi.ltd | 104.21.72.233 | IPv4 | h2 | ✅ 成功 | 62 |
cloudflare | | 217 | bowen.ns.cloudflare.com | 172.64.35.83 | IPv4 | h2 | ✅
成功 | 62 | cloudflare | | 222 | 172.67.243.218 | 172.67.243.218 | IPv4 | h2 |
✅ 成功 | 62 | cloudflare | | 224 | local-aria2-webui.masx200.ddns-ip.net |
172.67.157.182 | IPv4 | h2 | ✅ 成功 | 62 | cloudflare | | 253 | ip.sb |
104.26.12.31 | IPv4 | h2 | ✅ 成功 | 62 | cloudflare | | 302 | fbi.gov |
104.16.149.244 | IPv4 | h2 | ✅ 成功 | 62 | cloudflare | | 308 | 172.67.75.172 |
172.67.75.172 | IPv4 | h2 | ✅ 成功 | 62 | cloudflare | | 316 | eur.877774.xyz |
104.21.47.209 | IPv4 | h2 | ✅ 成功 | 62 | cloudflare | | 470 |
whatismyipaddress.com | 104.19.223.79 | IPv4 | h2 | ✅ 成功 | 62 | cloudflare |
| 477 | icook.tw | 2606:4700:10::ac42:9e73 | IPv6 | h2 | ✅ 成功 | 62 |
cloudflare |

### 延迟分布分析

- **超快 (<50ms)**: 4 条记录
- **快 (50-100ms)**: 96 条记录
- **正常 (100-200ms)**: 0 条记录
- **慢 (200-500ms)**: 0 条记录
- **很慢 (>500ms)**: 0 条记录

---

## 详细分析

### 按IP版本统计

- **IPv4 失败**: 11 次
- **IPv6 失败**: 0 次

### 按协议统计

- **none**: 11 次失败

---

## 建议和后续操作

1. **检查网络连接**: 确认网络连接稳定
2. **验证DNS解析**: 检查DNS服务器是否正常工作
3. **检查防火墙设置**: 确认防火墙没有阻止相关端口
4. **联系服务提供商**: 如果失败率较高，可能需要联系网络服务提供商
5. **重新运行测试**: 在网络条件改善后重新运行测试进行验证

---

_此报告由 HTTP/3 连接测试报告生成器自动生成_
