# HTTP/3 连接测试失败报告

## 报告概要

- **生成时间**: 2025/12/10 17:56:45
- **数据来源**: connectivity_results-20251210-175645.json
- **总测试数**: 485
- **失败测试数**: 10
- **成功测试数**: 475
- **失败率**: 2.06%
- **平均延迟**: 114.23ms
- **最小延迟**: 73ms
- **最大延迟**: 829ms

---

## 失败测试详情

### 📊 错误类型统计

- **连接超时: I/O超时**: 8 次 (80.0%)
- **DNS解析错误: 其他DNS错误**: 1 次 (10.0%)
- **连接超时: 上下文超时**: 1 次 (10.0%)

### 🔍 按错误类型分类的失败测试详情

#### 连接超时: I/O超时 (8 次测试)

| 序号 | 主机/域名        | 目标IP          | IP版本 | 协议 | 状态码 | 延迟(ms) | 服务器 | 错误信息                                  |
| ---- | ---------------- | --------------- | ------ | ---- | ------ | -------- | ------ | ----------------------------------------- |
| 5    | 175.212.207.13   | 175.212.207.13  | IPv4   | none | N/A    | 0        | N/A    | dial tcp 175.212.207.13:443: i/o timeout  |
| 6    | ct.877774.xyz    | ct.877774.xyz   | IPv4   | none | N/A    | 0        | N/A    | dial tcp ct.877774.xyz:443: i/o timeout   |
| 90   | 121.188.182.190  | 121.188.182.190 | IPv4   | none | N/A    | 0        | N/A    | dial tcp 121.188.182.190:443: i/o timeout |
| 118  | 115.22.115.218   | 115.22.115.218  | IPv4   | none | N/A    | 0        | N/A    | dial tcp 115.22.115.218:443: i/o timeout  |
| 158  | cfip.xxxxxxxx.tk | 198.41.212.130  | IPv4   | none | N/A    | 0        | N/A    | dial tcp 198.41.212.130:443: i/o timeout  |
| 171  | 52.76.110.129    | 52.76.110.129   | IPv4   | none | N/A    | 0        | N/A    | dial tcp 52.76.110.129:443: i/o timeout   |
| 215  | 111.171.108.67   | 111.171.108.67  | IPv4   | none | N/A    | 0        | N/A    | dial tcp 111.171.108.67:443: i/o timeout  |
| 292  | 119.194.220.146  | 119.194.220.146 | IPv4   | none | N/A    | 0        | N/A    | dial tcp 119.194.220.146:443: i/o timeout |

#### DNS解析错误: 其他DNS错误 (1 次测试)

| 序号 | 主机/域名       | 目标IP          | IP版本 | 协议 | 状态码 | 延迟(ms) | 服务器 | 错误信息                                                                                       |
| ---- | --------------- | --------------- | ------ | ---- | ------ | -------- | ------ | ---------------------------------------------------------------------------------------------- |
| 3    | 168.138.184.172 | 168.138.184.172 | IPv4   | h2   | N/A    | 0        | N/A    | Get "https://local-aria2-webui.masx200.ddns-ip.net:443/": remote error: tls: unrecognized name |

#### 连接超时: 上下文超时 (1 次测试)

| 序号 | 主机/域名      | 目标IP         | IP版本 | 协议 | 状态码 | 延迟(ms) | 服务器 | 错误信息                                                                            |
| ---- | -------------- | -------------- | ------ | ---- | ------ | -------- | ------ | ----------------------------------------------------------------------------------- |
| 172  | 20.247.137.183 | 20.247.137.183 | IPv4   | h2   | N/A    | 0        | N/A    | Get "https://local-aria2-webui.masx200.ddns-ip.net:443/": context deadline exceeded |

### 📈 错误分析总结

#### 主错误类型分布

- **连接超时**: 9 次 (90.0%)
- **DNS解析错误**: 1 次 (10.0%)

#### 错误模式分析

**超时集中度分析**: 共有 8 次超时，主要集中在IP段 175.212（1
次），可能存在网络路由问题或目标服务器负载过高

**协议协商分析**: 有 8 次失败是因为协议协商失败（protocol:
none），说明无法与目标建立HTTP/3或其他现代协议连接

**IP版本分析**:
所有失败的测试都使用IPv4，IPv6连接可能更稳定或目标服务器的IPv6配置更好

---

## 🚀 延迟最低的 100 条记录

以下显示了延迟最低的测试记录，包括成功和失败的测试：

| 序号 | 主机/域名 | 目标IP | IP版本 | 协议 | 状态 | 延迟(ms) | 服务器 |
|------|-----------|--------|--------|------|------|----------|--------|| 154 |
www.wto.org | 104.18.41.190 | IPv4 | h2 | ✅ 成功 | 73 | cloudflare | | 24 |
fbi.gov | 2606:4700::6810:94f4 | IPv6 | h2 | ✅ 成功 | 76 | cloudflare | | 309 |
cf.877771.xyz | 2606:4700:3033::ac43:98b7 | IPv6 | h2 | ✅ 成功 | 76 |
cloudflare | | 359 | yx-auto.pages.dev | 2606:4700:3031::ac43:868b | IPv6 | h2 |
✅ 成功 | 76 | cloudflare | | 88 | local-aria2-webui.masx200.ddns-ip.net |
2606:4700:3031::ac43:9db6 | IPv6 | h2 | ✅ 成功 | 77 | cloudflare | | 124 |
singapore.com | 2606:4700:20::ac43:4bc2 | IPv6 | h2 | ✅ 成功 | 77 | cloudflare
| | 152 | cf.zhetengsha.eu.org | 2606:4700:310c::ac42:2c4d | IPv6 | h2 | ✅ 成功
| 77 | cloudflare | | 176 | www.hugedomains.com | 2606:4700:20::681a:625 | IPv6
| h2 | ✅ 成功 | 77 | cloudflare | | 277 | ip.sb | 104.26.12.31 | IPv4 | h2 | ✅
成功 | 77 | cloudflare | | 12 | cloudflare-ip.mofashi.ltd |
2606:4700:3037::6815:48e9 | IPv6 | h2 | ✅ 成功 | 78 | cloudflare | | 18 |
palera.in | 104.21.58.72 | IPv4 | h2 | ✅ 成功 | 78 | cloudflare | | 27 |
[2606:4700:440f::53aa:4126] | 2606:4700:440f::53aa:4126 | IPv6 | h2 | ✅ 成功 |
78 | cloudflare | | 56 | www.udemy.com | 2606:4700::6810:8eed | IPv6 | h2 | ✅
成功 | 78 | cloudflare | | 73 | ct.877774.xyz | 172.64.229.195 | IPv4 | h2 | ✅
成功 | 78 | cloudflare | | 120 | singapore.com | 172.67.75.194 | IPv4 | h2 | ✅
成功 | 78 | cloudflare | | 208 | bestcf.030101.xyz | 104.17.61.18 | IPv4 | h2 |
✅ 成功 | 78 | cloudflare | | 251 | zread.ai | 2606:4700:3032::ac43:ca4e | IPv6
| h2 | ✅ 成功 | 78 | cloudflare | | 275 | ip.sb | 104.26.13.31 | IPv4 | h2 | ✅
成功 | 78 | cloudflare | | 276 | ip.sb | 172.67.75.172 | IPv4 | h2 | ✅ 成功 |
78 | cloudflare | | 278 | ip.sb | 2606:4700:20::681a:c1f | IPv6 | h2 | ✅ 成功 |
78 | cloudflare | | 308 | cf.877771.xyz | 172.67.152.183 | IPv4 | h2 | ✅ 成功 |
78 | cloudflare | | 381 | cf.090227.xyz | 172.66.44.77 | IPv4 | h2 | ✅ 成功 |
78 | cloudflare | | 383 | cf.090227.xyz | 2606:4700:310c::ac42:2c4d | IPv6 | h2
| ✅ 成功 | 78 | cloudflare | | 397 | cf.877774.xyz | cf.877774.xyz | IPv4 | h2
| ✅ 成功 | 78 | cloudflare | | 411 | 456.cloudflare.182682.xyz | 104.26.9.160 |
IPv4 | h2 | ✅ 成功 | 78 | cloudflare | | 44 | japan.com | 104.26.5.60 | IPv4 |
h2 | ✅ 成功 | 79 | cloudflare | | 49 | japan.com | 2606:4700:20::ac43:465c |
IPv6 | h2 | ✅ 成功 | 79 | cloudflare | | 77 | yx-auto.pages.dev | 172.66.44.144
| IPv4 | h2 | ✅ 成功 | 79 | cloudflare | | 79 | yx-auto.pages.dev |
2606:4700:310c::ac42:2f70 | IPv6 | h2 | ✅ 成功 | 79 | cloudflare | | 80 |
yx-auto.pages.dev | 2606:4700:310c::ac42:2c90 | IPv6 | h2 | ✅ 成功 | 79 |
cloudflare | | 128 | whatismyipaddress.com | 2606:4700::6813:de4f | IPv6 | h2 |
✅ 成功 | 79 | cloudflare | | 146 | lewis.ns.cloudflare.com |
2606:4700:58::a29f:2c9f | IPv6 | h2 | ✅ 成功 | 79 | cloudflare | | 149 |
[2606:4700:4409::5b5b:7758] | 2606:4700:4409::5b5b:7758 | IPv6 | h2 | ✅ 成功 |
79 | cloudflare | | 165 | www.csgo.com | 195.85.59.95 | IPv4 | h2 | ✅ 成功 | 79
| cloudflare | | 170 | icook.hk | 2606:4700:3037::ac43:a168 | IPv6 | h2 | ✅
成功 | 79 | cloudflare | | 183 | julio.ns.cloudflare.com |
2803:f800:50::6ca2:c3d1 | IPv6 | h2 | ✅ 成功 | 79 | cloudflare | | 196 |
[2606:4700:440b::3e6e:5f06] | 2606:4700:440b::3e6e:5f06 | IPv6 | h2 | ✅ 成功 |
79 | cloudflare | | 199 | tasteatlas.com | 2606:4700::6811:2469 | IPv6 | h2 | ✅
成功 | 79 | cloudflare | | 253 | www.whatismyip.com | 104.26.13.23 | IPv4 | h2 |
✅ 成功 | 79 | cloudflare | | 267 | cu.877774.xyz | 104.26.4.111 | IPv4 | h2 |
✅ 成功 | 79 | cloudflare | | 279 | ip.sb | 2606:4700:20::ac43:4bac | IPv6 | h2
| ✅ 成功 | 79 | cloudflare | | 284 | yx-auto.pages.dev |
2606:4700:3034::6815:9e6 | IPv6 | h2 | ✅ 成功 | 79 | cloudflare | | 380 |
172.64.154.18 | 172.64.154.18 | IPv4 | h2 | ✅ 成功 | 79 | cloudflare | | 384 |
cf.090227.xyz | 2606:4700:310c::ac42:2fb3 | IPv6 | h2 | ✅ 成功 | 79 |
cloudflare | | 396 | ae8a9c24-83de.masx200.ddns-ip.net |
2606:4700:3030::6815:e29 | IPv6 | h2 | ✅ 成功 | 79 | cloudflare | | 11 |
cloudflare-ip.mofashi.ltd | 104.21.72.233 | IPv4 | h2 | ✅ 成功 | 80 |
cloudflare | | 29 | [2606:4700:4403::7357:544f] | 2606:4700:4403::7357:544f |
IPv6 | h2 | ✅ 成功 | 80 | cloudflare | | 32 | toy-people.com | 104.26.2.36 |
IPv4 | h2 | ✅ 成功 | 80 | cloudflare | | 46 | japan.com | 104.26.4.60 | IPv4 |
h2 | ✅ 成功 | 80 | cloudflare | | 59 | steamdb.info | 2606:4700:10::ac42:affa |
IPv6 | h2 | ✅ 成功 | 80 | cloudflare | | 60 | steamdb.info |
2606:4700:10::6814:22d4 | IPv6 | h2 | ✅ 成功 | 80 | cloudflare | | 70 |
ct.877774.xyz | 172.64.229.44 | IPv4 | h2 | ✅ 成功 | 80 | cloudflare | | 83 |
www.visa.com.hk | 104.18.20.69 | IPv4 | h2 | ✅ 成功 | 80 | cloudflare | | 134 |
cf.877774.xyz | 2606:4700:4406::ac40:9242 | IPv6 | h2 | ✅ 成功 | 80 |
cloudflare | | 155 | www.wto.org | 172.64.146.66 | IPv4 | h2 | ✅ 成功 | 80 |
cloudflare | | 157 | www.wto.org | 2a06:98c1:3102::6812:29be | IPv6 | h2 | ✅
成功 | 80 | cloudflare | | 166 | www.csgo.com | 195.85.59.161 | IPv4 | h2 | ✅
成功 | 80 | cloudflare | | 310 | cf.877771.xyz | 2606:4700:3033::6815:50b4 |
IPv6 | h2 | ✅ 成功 | 80 | cloudflare | | 343 | www.pcmag.com |
2606:4700::6810:1476 | IPv6 | h2 | ✅ 成功 | 80 | cloudflare | | 355 |
www.ipget.net | 2606:4700:3031::ac43:cf1a | IPv6 | h2 | ✅ 成功 | 80 |
cloudflare | | 370 | asia.877774.xyz | 104.17.139.62 | IPv4 | h2 | ✅ 成功 | 80
| cloudflare | | 419 | braden.ns.cloudflare.com | 2606:4700:58::a29f:2ca9 | IPv6
| h2 | ✅ 成功 | 80 | cloudflare | | 426 | stock.hostmonit.com | 104.21.7.193 |
IPv4 | h2 | ✅ 成功 | 80 | cloudflare | | 462 | cmcc.877774.xyz | 104.16.148.7 |
IPv4 | h2 | ✅ 成功 | 80 | cloudflare | | 15 | comicabc.com | 172.67.174.21 |
IPv4 | h2 | ✅ 成功 | 81 | cloudflare | | 35 | toy-people.com |
2606:4700:20::681a:224 | IPv6 | h2 | ✅ 成功 | 81 | cloudflare | | 39 |
www.7749tv.com | 172.67.198.170 | IPv4 | h2 | ✅ 成功 | 81 | cloudflare | | 48 |
japan.com | 2606:4700:20::681a:53c | IPv6 | h2 | ✅ 成功 | 81 | cloudflare | |
93 | iplocation.io | 104.26.10.222 | IPv4 | h2 | ✅ 成功 | 81 | cloudflare | |
96 | iplocation.io | 2606:4700:20::681a:ade | IPv6 | h2 | ✅ 成功 | 81 |
cloudflare | | 108 | cris.ns.cloudflare.com | 2a06:98c1:50::ac40:23ca | IPv6 |
h2 | ✅ 成功 | 81 | cloudflare | | 122 | singapore.com | 2606:4700:20::681a:d8c
| IPv6 | h2 | ✅ 成功 | 81 | cloudflare | | 126 | whatismyipaddress.com |
104.19.223.79 | IPv4 | h2 | ✅ 成功 | 81 | cloudflare | | 150 |
cf.zhetengsha.eu.org | 172.66.47.179 | IPv4 | h2 | ✅ 成功 | 81 | cloudflare | |
175 | www.hugedomains.com | 172.67.70.191 | IPv4 | h2 | ✅ 成功 | 81 |
cloudflare | | 177 | www.hugedomains.com | 2606:4700:20::ac43:46bf | IPv6 | h2 |
✅ 成功 | 81 | cloudflare | | 178 | www.hugedomains.com | 2606:4700:20::681a:725
| IPv6 | h2 | ✅ 成功 | 81 | cloudflare | | 190 | ip.gs | 172.67.160.28 | IPv4 |
h2 | ✅ 成功 | 81 | cloudflare | | 249 | zread.ai | 104.21.76.240 | IPv4 | h2 |
✅ 成功 | 81 | cloudflare | | 319 | www.4chan.org | 104.16.228.229 | IPv4 | h2 |
✅ 成功 | 81 | cloudflare | | 336 | time.is | 2606:4700:20::681a:c36 | IPv6 | h2
| ✅ 成功 | 81 | cloudflare | | 342 | www.pcmag.com | 2606:4700::6810:1576 |
IPv6 | h2 | ✅ 成功 | 81 | cloudflare | | 391 | 104.16.223.179 | 104.16.223.179
| IPv4 | h2 | ✅ 成功 | 81 | cloudflare | | 412 | 456.cloudflare.182682.xyz |
2606:4700:20::ac43:4bd0 | IPv6 | h2 | ✅ 成功 | 81 | cloudflare | | 423 |
ipinfo.in | 172.67.198.203 | IPv4 | h2 | ✅ 成功 | 81 | cloudflare | | 461 |
cmcc.877774.xyz | 104.16.148.6 | IPv4 | h2 | ✅ 成功 | 81 | cloudflare | | 466 |
cmcc.877774.xyz | 104.16.148.11 | IPv4 | h2 | ✅ 成功 | 81 | cloudflare | | 61 |
104.26.13.31 | 104.26.13.31 | IPv4 | h2 | ✅ 成功 | 82 | cloudflare | | 66 |
www.ipchicken.com | 104.26.7.112 | IPv4 | h2 | ✅ 成功 | 82 | cloudflare | | 78
| yx-auto.pages.dev | 172.66.47.112 | IPv4 | h2 | ✅ 成功 | 82 | cloudflare | |
85 | local-aria2-webui.masx200.ddns-ip.net | 104.21.14.41 | IPv4 | h2 | ✅ 成功
| 82 | cloudflare | | 92 | 104.16.61.163 | 104.16.61.163 | IPv4 | h2 | ✅ 成功 |
82 | cloudflare | | 100 | icook.tw | 172.66.158.115 | IPv4 | h2 | ✅ 成功 | 82 |
cloudflare | | 110 | 172.67.181.209 | 172.67.181.209 | IPv4 | h2 | ✅ 成功 | 82
| cloudflare | | 114 | ashton.ns.cloudflare.com | 2606:4700:58::a29f:2cad | IPv6
| h2 | ✅ 成功 | 82 | cloudflare | | 140 | dylan.ns.cloudflare.com |
2606:4700:58::a29f:2cbb | IPv6 | h2 | ✅ 成功 | 82 | cloudflare | | 141 |
dylan.ns.cloudflare.com | 2803:f800:50::6ca2:c3bb | IPv6 | h2 | ✅ 成功 | 82 |
cloudflare | | 148 | lewis.ns.cloudflare.com | 2a06:98c1:50::ac40:239f | IPv6 |
h2 | ✅ 成功 | 82 | cloudflare | | 188 | 104.17.68.85 | 104.17.68.85 | IPv4 | h2
| ✅ 成功 | 82 | cloudflare | | 192 | ip.gs | 2606:4700:3035::ac43:a01c | IPv6 |
h2 | ✅ 成功 | 82 | cloudflare |

### 延迟分布分析

- **超快 (<50ms)**: 0 条记录
- **快 (50-100ms)**: 100 条记录
- **正常 (100-200ms)**: 0 条记录
- **慢 (200-500ms)**: 0 条记录
- **很慢 (>500ms)**: 0 条记录

---

## 详细分析

### 按IP版本统计

- **IPv4 失败**: 10 次
- **IPv6 失败**: 0 次

### 按协议统计

- **h2**: 2 次失败
- **none**: 8 次失败

---

## 建议和后续操作

1. **检查网络连接**: 确认网络连接稳定
2. **验证DNS解析**: 检查DNS服务器是否正常工作
3. **检查防火墙设置**: 确认防火墙没有阻止相关端口
4. **联系服务提供商**: 如果失败率较高，可能需要联系网络服务提供商
5. **重新运行测试**: 在网络条件改善后重新运行测试进行验证

---

_此报告由 HTTP/3 连接测试报告生成器自动生成_
