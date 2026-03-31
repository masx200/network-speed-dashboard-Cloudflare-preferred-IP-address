# HTTP/3 连接测试失败报告

## 报告概要

- **生成时间**: 2025/12/14 21:27:55
- **数据来源**: connectivity_results-20251214-132742.json
- **总测试数**: 434
- **失败测试数**: 9
- **成功测试数**: 425
- **失败率**: 2.07%
- **平均延迟**: 1118.04ms
- **最小延迟**: 150ms
- **最大延迟**: 9217ms

## 🌐 当前测试环境信息

- **获取时间**: 2025/12/14 21:27:55
- **IP地址**: 2409:891f:8363:505:bccc:3eff:fe54:8a5b
- **国家/地区**: China (CN)
- **ASN**: AS24400
- **网络组织**: Shanghai Mobile Communications Co.,Ltd.
- **网络域名**: chinamobile.com
- **大洲**: Asia (AS)
- **地理坐标**: 34.7732, 113.722
- **时区**: Asia/Shanghai
- **数据源**: combined

---

---

## 失败测试详情

### 📊 错误类型统计

- **连接超时: I/O超时**: 6 次 (66.7%)
- **DNS解析错误: 其他DNS错误**: 2 次 (22.2%)
- **连接超时: 上下文超时**: 1 次 (11.1%)

### 🔍 按错误类型分类的失败测试详情

#### 连接超时: I/O超时 (6 次测试)

| 序号 | 主机/域名                | 目标IP          | IP版本 | 协议 | 状态码 | 延迟(ms) | 服务器 | 错误信息                                  |
| ---- | ------------------------ | --------------- | ------ | ---- | ------ | -------- | ------ | ----------------------------------------- |
| 9    | 141.147.185.63           | 141.147.185.63  | IPv4   | none | N/A    | 0        | N/A    | dial tcp 141.147.185.63:443: i/o timeout  |
| 58   | cf.877774.xyz            | cf.877774.xyz   | IPv4   | none | N/A    | 0        | N/A    | dial tcp cf.877774.xyz:443: i/o timeout   |
| 367  | ct.877774.xyz            | ct.877774.xyz   | IPv4   | none | N/A    | 0        | N/A    | dial tcp ct.877774.xyz:443: i/o timeout   |
| 419  | 3.0.50.69                | 3.0.50.69       | IPv4   | none | N/A    | 0        | N/A    | dial tcp 3.0.50.69:443: i/o timeout       |
| 428  | cfip.xxxxxxxx.tk         | 198.41.212.130  | IPv4   | none | N/A    | 0        | N/A    | dial tcp 198.41.212.130:443: i/o timeout  |
| 430  | trevor.ns.cloudflare.com | 108.162.195.154 | IPv4   | none | N/A    | 0        | N/A    | dial tcp 108.162.195.154:443: i/o timeout |

#### DNS解析错误: 其他DNS错误 (2 次测试)

| 序号 | 主机/域名     | 目标IP  | IP版本  | 协议 | 状态码 | 延迟(ms) | 服务器 | 错误信息      |
| ---- | ------------- | ------- | ------- | ---- | ------ | -------- | ------ | ------------- |
| 60   | cf.877771.xyz | Unknown | Unknown | none | N/A    | 0        | N/A    | DNS解析无结果 |
| 61   | www.visa.cn   | Unknown | Unknown | none | N/A    | 0        | N/A    | DNS解析无结果 |

#### 连接超时: 上下文超时 (1 次测试)

| 序号 | 主机/域名                | 目标IP                  | IP版本 | 协议 | 状态码 | 延迟(ms) | 服务器 | 错误信息                                                                            |
| ---- | ------------------------ | ----------------------- | ------ | ---- | ------ | -------- | ------ | ----------------------------------------------------------------------------------- |
| 71   | huxley.ns.cloudflare.com | 2606:4700:58::a29f:2cbc | IPv6   | h2   | N/A    | 0        | N/A    | Get "https://local-aria2-webui.masx200.ddns-ip.net:443/": context deadline exceeded |

### 📈 错误分析总结

#### 主错误类型分布

- **连接超时**: 7 次 (77.8%)
- **DNS解析错误**: 2 次 (22.2%)

#### 错误模式分析

**超时集中度分析**: 共有 6 次超时，主要集中在IP段 141.147（1
次），可能存在网络路由问题或目标服务器负载过高

**协议协商分析**: 有 8 次失败是因为协议协商失败（protocol:
none），说明无法与目标建立HTTP/3或其他现代协议连接

**IP版本分析**: IPv4失败 6 次，IPv6失败 1 次，两种协议都存在问题

---

## 🚀 延迟最低的 100 条记录

以下显示了延迟最低的测试记录，包括成功和失败的测试：

| 序号 | 主机/域名                             | 目标IP                    | IP版本 | 协议 | 状态    | 延迟(ms) | 服务器     |
| ---- | ------------------------------------- | ------------------------- | ------ | ---- | ------- | -------- | ---------- |
| 385  | cmcc.877774.xyz                       | 104.16.148.8              | IPv4   | h3   | ✅ 成功 | 150      | cloudflare |
| 370  | ct.877774.xyz                         | 172.64.229.161            | IPv4   | h3   | ✅ 成功 | 153      | cloudflare |
| 403  | cmcc.877774.xyz                       | 104.16.149.244            | IPv4   | h3   | ✅ 成功 | 158      | cloudflare |
| 388  | cmcc.877774.xyz                       | 104.16.148.11             | IPv4   | h3   | ✅ 成功 | 160      | cloudflare |
| 383  | cmcc.877774.xyz                       | 104.16.148.6              | IPv4   | h3   | ✅ 成功 | 162      | cloudflare |
| 369  | ct.877774.xyz                         | 172.64.229.173            | IPv4   | h3   | ✅ 成功 | 164      | cloudflare |
| 386  | cmcc.877774.xyz                       | 104.16.148.9              | IPv4   | h3   | ✅ 成功 | 164      | cloudflare |
| 384  | cmcc.877774.xyz                       | 104.16.148.7              | IPv4   | h3   | ✅ 成功 | 165      | cloudflare |
| 372  | ct.877774.xyz                         | 172.64.229.195            | IPv4   | h3   | ✅ 成功 | 166      | cloudflare |
| 405  | cmcc.877774.xyz                       | 104.16.148.2              | IPv4   | h3   | ✅ 成功 | 166      | cloudflare |
| 235  | 104.18.42.26                          | 104.18.42.26              | IPv4   | h3   | ✅ 成功 | 167      | cloudflare |
| 153  | bestcf.030101.xyz                     | 104.19.61.113             | IPv4   | h3   | ✅ 成功 | 170      | cloudflare |
| 180  | cf.090227.xyz                         | 172.64.144.82             | IPv4   | h3   | ✅ 成功 | 170      | cloudflare |
| 422  | cfip.xxxxxxxx.tk                      | 104.16.241.229            | IPv4   | h3   | ✅ 成功 | 170      | cloudflare |
| 336  | 172.64.154.18                         | 172.64.154.18             | IPv4   | h3   | ✅ 成功 | 171      | cloudflare |
| 373  | ct.877774.xyz                         | 172.64.229.217            | IPv4   | h3   | ✅ 成功 | 171      | cloudflare |
| 393  | cmcc.877774.xyz                       | 104.16.149.3              | IPv4   | h3   | ✅ 成功 | 173      | cloudflare |
| 176  | fbi.gov                               | 104.16.149.244            | IPv4   | h3   | ✅ 成功 | 174      | cloudflare |
| 387  | cmcc.877774.xyz                       | 104.16.148.10             | IPv4   | h3   | ✅ 成功 | 176      | cloudflare |
| 396  | cmcc.877774.xyz                       | 104.16.149.6              | IPv4   | h3   | ✅ 成功 | 177      | cloudflare |
| 394  | cmcc.877774.xyz                       | 104.16.149.4              | IPv4   | h3   | ✅ 成功 | 178      | cloudflare |
| 395  | cmcc.877774.xyz                       | 104.16.149.5              | IPv4   | h3   | ✅ 成功 | 178      | cloudflare |
| 400  | cmcc.877774.xyz                       | 104.16.149.10             | IPv4   | h3   | ✅ 成功 | 178      | cloudflare |
| 401  | cmcc.877774.xyz                       | 104.16.149.11             | IPv4   | h3   | ✅ 成功 | 178      | cloudflare |
| 392  | cmcc.877774.xyz                       | 104.16.149.2              | IPv4   | h3   | ✅ 成功 | 179      | cloudflare |
| 397  | cmcc.877774.xyz                       | 104.16.149.7              | IPv4   | h3   | ✅ 成功 | 182      | cloudflare |
| 371  | ct.877774.xyz                         | 172.64.229.44             | IPv4   | h3   | ✅ 成功 | 185      | cloudflare |
| 399  | cmcc.877774.xyz                       | 104.16.149.9              | IPv4   | h3   | ✅ 成功 | 185      | cloudflare |
| 167  | cf.zhetengsha.eu.org                  | 104.18.35.15              | IPv4   | h3   | ✅ 成功 | 186      | cloudflare |
| 398  | cmcc.877774.xyz                       | 104.16.149.8              | IPv4   | h3   | ✅ 成功 | 192      | cloudflare |
| 376  | ct.877774.xyz                         | 172.64.229.174            | IPv4   | h3   | ✅ 成功 | 195      | cloudflare |
| 154  | bestcf.030101.xyz                     | 104.19.42.208             | IPv4   | h3   | ✅ 成功 | 198      | cloudflare |
| 375  | ct.877774.xyz                         | 172.64.229.185            | IPv4   | h3   | ✅ 成功 | 201      | cloudflare |
| 175  | fbi.gov                               | 104.16.148.244            | IPv4   | h3   | ✅ 成功 | 203      | cloudflare |
| 374  | ct.877774.xyz                         | 172.64.229.236            | IPv4   | h3   | ✅ 成功 | 203      | cloudflare |
| 231  | whatismyipaddress.com                 | 104.19.222.79             | IPv4   | h3   | ✅ 成功 | 204      | cloudflare |
| 389  | cmcc.877774.xyz                       | 104.16.148.12             | IPv4   | h3   | ✅ 成功 | 204      | cloudflare |
| 391  | cmcc.877774.xyz                       | 104.16.149.1              | IPv4   | h3   | ✅ 成功 | 208      | cloudflare |
| 414  | wilson.ns.cloudflare.com              | 108.162.195.110           | IPv4   | h3   | ✅ 成功 | 209      | cloudflare |
| 200  | benedict.ns.cloudflare.com            | 108.162.195.205           | IPv4   | h3   | ✅ 成功 | 212      | cloudflare |
| 324  | otto.ns.cloudflare.com                | 172.64.35.135             | IPv4   | h3   | ✅ 成功 | 214      | cloudflare |
| 423  | cfip.xxxxxxxx.tk                      | 104.17.127.110            | IPv4   | h3   | ✅ 成功 | 216      | cloudflare |
| 408  | cmcc.877774.xyz                       | 104.16.148.5              | IPv4   | h3   | ✅ 成功 | 219      | cloudflare |
| 390  | cmcc.877774.xyz                       | 104.16.148.244            | IPv4   | h3   | ✅ 成功 | 220      | cloudflare |
| 427  | cfip.xxxxxxxx.tk                      | 190.93.246.67             | IPv4   | h3   | ✅ 成功 | 220      | cloudflare |
| 181  | cf.090227.xyz                         | 104.18.43.174             | IPv4   | h3   | ✅ 成功 | 225      | cloudflare |
| 141  | asia.877774.xyz                       | 104.17.139.62             | IPv4   | h3   | ✅ 成功 | 232      | cloudflare |
| 406  | cmcc.877774.xyz                       | 104.16.148.3              | IPv4   | h3   | ✅ 成功 | 247      | cloudflare |
| 165  | saas.sin.fan                          | 162.159.36.5              | IPv4   | h3   | ✅ 成功 | 250      | cloudflare |
| 402  | cmcc.877774.xyz                       | 104.16.149.12             | IPv4   | h3   | ✅ 成功 | 252      | cloudflare |
| 407  | cmcc.877774.xyz                       | 104.16.148.4              | IPv4   | h3   | ✅ 成功 | 266      | cloudflare |
| 168  | cf.zhetengsha.eu.org                  | 172.64.152.241            | IPv4   | h3   | ✅ 成功 | 271      | cloudflare |
| 87   | na.877774.xyz                         | 104.18.38.235             | IPv4   | h3   | ✅ 成功 | 284      | cloudflare |
| 320  | [2606:4700:83bd::7d8:2b47]            | 2606:4700:83bd::7d8:2b47  | IPv6   | h3   | ✅ 成功 | 303      | cloudflare |
| 248  | cf.877774.xyz                         | 172.64.146.66             | IPv4   | h3   | ✅ 成功 | 306      | cloudflare |
| 101  | pranab.ns.cloudflare.com              | 108.162.195.199           | IPv4   | h3   | ✅ 成功 | 307      | cloudflare |
| 322  | 104.19.223.58                         | 104.19.223.58             | IPv4   | h3   | ✅ 成功 | 308      | cloudflare |
| 305  | [2606:4700:83be::11:74f]              | 2606:4700:83be::11:74f    | IPv6   | h3   | ✅ 成功 | 327      | cloudflare |
| 127  | kyree.ns.cloudflare.com               | 172.64.35.207             | IPv4   | h3   | ✅ 成功 | 340      | cloudflare |
| 404  | cmcc.877774.xyz                       | 104.16.148.1              | IPv4   | h3   | ✅ 成功 | 370      | cloudflare |
| 353  | abdullah.ns.cloudflare.com            | 108.162.195.203           | IPv4   | h3   | ✅ 成功 | 377      | cloudflare |
| 267  | ashton.ns.cloudflare.com              | 172.64.35.173             | IPv4   | h3   | ✅ 成功 | 380      | cloudflare |
| 135  | www.whatismyip.com                    | 104.26.13.23              | IPv4   | h3   | ✅ 成功 | 385      | cloudflare |
| 273  | tasteatlas.com                        | 104.17.37.105             | IPv4   | h3   | ✅ 成功 | 390      | cloudflare |
| 247  | 162.159.133.85                        | 162.159.133.85            | IPv4   | h3   | ✅ 成功 | 396      | cloudflare |
| 317  | eur.877774.xyz                        | 104.21.26.150             | IPv4   | h3   | ✅ 成功 | 406      | cloudflare |
| 272  | tasteatlas.com                        | 104.17.36.105             | IPv4   | h3   | ✅ 成功 | 407      | cloudflare |
| 345  | ifconfig.co                           | 2606:4700:3037::6815:365b | IPv6   | h3   | ✅ 成功 | 407      | cloudflare |
| 298  | 104.18.37.40                          | 104.18.37.40              | IPv4   | h3   | ✅ 成功 | 411      | cloudflare |
| 161  | toy-people.com                        | 2606:4700:20::681a:324    | IPv6   | h3   | ✅ 成功 | 412      | cloudflare |
| 232  | whatismyipaddress.com                 | 104.19.223.79             | IPv4   | h3   | ✅ 成功 | 414      | cloudflare |
| 279  | dnschecker.org                        | 2606:4700:20::681a:659    | IPv6   | h3   | ✅ 成功 | 416      | cloudflare |
| 363  | www.wto.org                           | 172.64.146.66             | IPv4   | h3   | ✅ 成功 | 419      | cloudflare |
| 259  | local-aria2-webui.masx200.ddns-ip.net | 2606:4700:3031::ac43:9db6 | IPv6   | h2   | ✅ 成功 | 421      | cloudflare |
| 137  | www.whatismyip.com                    | 2606:4700:20::681a:c17    | IPv6   | h3   | ✅ 成功 | 426      | cloudflare |
| 348  | comicabc.com                          | 104.21.64.10              | IPv4   | h3   | ✅ 成功 | 426      | cloudflare |
| 218  | ip.gs                                 | 172.67.160.28             | IPv4   | h3   | ✅ 成功 | 428      | cloudflare |
| 227  | cloudflare-ip.mofashi.ltd             | 172.67.155.172            | IPv4   | h3   | ✅ 成功 | 428      | cloudflare |
| 415  | wilson.ns.cloudflare.com              | 172.64.35.110             | IPv4   | h3   | ✅ 成功 | 428      | cloudflare |
| 192  | 104.16.61.163                         | 104.16.61.163             | IPv4   | h3   | ✅ 成功 | 429      | cloudflare |
| 276  | dnschecker.org                        | 104.26.6.89               | IPv4   | h3   | ✅ 成功 | 430      | cloudflare |
| 319  | eur.877774.xyz                        | 104.21.47.209             | IPv4   | h3   | ✅ 成功 | 430      | cloudflare |
| 278  | dnschecker.org                        | 104.26.7.89               | IPv4   | h3   | ✅ 成功 | 432      | cloudflare |
| 343  | ifconfig.co                           | 172.67.168.106            | IPv4   | h3   | ✅ 成功 | 434      | cloudflare |
| 347  | comicabc.com                          | 172.67.174.21             | IPv4   | h3   | ✅ 成功 | 435      | cloudflare |
| 213  | palera.in                             | 172.67.157.122            | IPv4   | h3   | ✅ 成功 | 437      | cloudflare |
| 195  | bowen.ns.cloudflare.com               | 108.162.195.83            | IPv4   | h3   | ✅ 成功 | 438      | cloudflare |
| 409  | www.pcmag.com                         | 104.16.20.118             | IPv4   | h3   | ✅ 成功 | 439      | cloudflare |
| 255  | 456.cloudflare.182682.xyz             | 2606:4700:20::ac43:4bd0   | IPv6   | h3   | ✅ 成功 | 441      | cloudflare |
| 171  | xn--b6gac.eu.org                      | 104.21.90.78              | IPv4   | h3   | ✅ 成功 | 443      | cloudflare |
| 253  | 456.cloudflare.182682.xyz             | 172.67.75.208             | IPv4   | h3   | ✅ 成功 | 443      | cloudflare |
| 311  | japan.com                             | 104.26.4.60               | IPv4   | h3   | ✅ 成功 | 443      | cloudflare |
| 74   | www.visa.com.sg                       | 104.18.12.229             | IPv4   | h3   | ✅ 成功 | 447      | cloudflare |
| 266  | ashton.ns.cloudflare.com              | 108.162.195.173           | IPv4   | h3   | ✅ 成功 | 448      | cloudflare |
| 67   | www.okcupid.com                       | 104.18.160.63             | IPv4   | h3   | ✅ 成功 | 449      | cloudflare |
| 357  | stock.hostmonit.com                   | 104.21.7.193              | IPv4   | h3   | ✅ 成功 | 449      | cloudflare |
| 381  | www.csgo.com                          | 195.85.59.161             | IPv4   | h3   | ✅ 成功 | 449      | cloudflare |
| 149  | zread.ai                              | 104.21.76.240             | IPv4   | h3   | ✅ 成功 | 450      | cloudflare |
| 243  | 172.67.106.26                         | 172.67.106.26             | IPv4   | h3   | ✅ 成功 | 450      | cloudflare |
| 287  | 104.17.142.12                         | 104.17.142.12             | IPv4   | h3   | ✅ 成功 | 450      | cloudflare |

### 延迟分布分析

- **超快 (<50ms)**: 0 条记录
- **快 (50-100ms)**: 0 条记录
- **正常 (100-200ms)**: 32 条记录
- **慢 (200-500ms)**: 68 条记录
- **很慢 (>500ms)**: 0 条记录

---

## 详细分析

### 按IP版本统计

- **IPv4 失败**: 6 次
- **IPv6 失败**: 1 次

### 按协议统计

- **none**: 8 次失败
- **h2**: 1 次失败

---

## 建议和后续操作

1. **检查网络连接**: 确认网络连接稳定
2. **验证DNS解析**: 检查DNS服务器是否正常工作
3. **检查防火墙设置**: 确认防火墙没有阻止相关端口
4. **联系服务提供商**: 如果失败率较高，可能需要联系网络服务提供商
5. **重新运行测试**: 在网络条件改善后重新运行测试进行验证

---

_此报告由 HTTP/3 连接测试报告生成器自动生成_
