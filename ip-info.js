#!/usr/bin/env node

import { exec } from "child_process";
import { promisify } from "util";

const execAsync = promisify(exec);

/**
 * 获取当前IP地址信息
 * 使用多个API服务获取IP地理位置信息
 */
class IPInfoFetcher {
  constructor() {
    this.ipinfo = {
      ip: null,
      asn: null,
      as_name: null,
      as_domain: null,
      country_code: null,
      country: null,
      continent_code: null,
      continent: null,
      latitude: null,
      longitude: null,
      time_zone: null,
      org: null,
      user_agent: null,
      source: "unknown",
    };
  }

  /**
   * 使用 curl 调用 ipinfo.io API
   */
  async fetchFromIPInfo() {
    try {
      console.log("正在从 ipinfo.io 获取IP信息...");
      const { stdout } = await execAsync(
        'curl -s https://api.ipinfo.io/lite/me -H "Authorization: Bearer e1d992dda9d73e"',
      );

      const data = JSON.parse(stdout);

      this.ipinfo = {
        ...this.ipinfo,
        ip: data.ip,
        asn: data.asn,
        as_name: data.as_name,
        as_domain: data.as_domain,
        country_code: data.country_code,
        country: data.country,
        continent_code: data.continent_code,
        continent: data.continent,
        source: "ipinfo.io",
      };

      console.log(`✅ ipinfo.io 获取成功: ${data.ip} (${data.country})`);
      return true;
    } catch (error) {
      console.error("❌ ipinfo.io 获取失败:", error.message);
      return false;
    }
  }

  /**
   * 使用 curl 调用 ifconfig.co API
   */
  async fetchFromIfConfig() {
    try {
      console.log("正在从 ifconfig.co 获取IP信息...");
      const { stdout } = await execAsync("curl -s https://ifconfig.co/json");

      const data = JSON.parse(stdout);

      this.ipinfo = {
        ...this.ipinfo,
        ip: data.ip,
        country: data.country,
        country_code: data.country_iso,
        latitude: data.latitude,
        longitude: data.longitude,
        time_zone: data.time_zone,
        asn: data.asn,
        as_name: data.asn_org,
        user_agent: data.user_agent,
        source: this.ipinfo.source === "unknown" ? "ifconfig.co" : "combined",
      };

      console.log(`✅ ifconfig.co 获取成功: ${data.ip} (${data.country})`);
      return true;
    } catch (error) {
      console.error("❌ ifconfig.co 获取失败:", error.message);
      return false;
    }
  }

  /**
   * 获取IP信息的主要方法
   * 依次尝试不同的API服务
   */
  async fetchIPInfo() {
    console.log("🔍 开始获取当前IP地址信息...");

    let success = false;

    // 首先尝试 ipinfo.io
    success = await this.fetchFromIPInfo();

    // 如果失败，尝试 ifconfig.co
    if (!success) {
      success = await this.fetchFromIfConfig();
    } // 如果 ipinfo.io 成功，也尝试 ifconfig.co 来获取更多信息
    else {
      await this.fetchFromIfConfig();
    }

    if (success) {
      console.log("✅ IP信息获取完成");
      console.log(`   IP地址: ${this.ipinfo.ip}`);
      console.log(
        `   国家: ${this.ipinfo.country} (${this.ipinfo.country_code})`,
      );
      console.log(`   ASN: ${this.ipinfo.asn}`);
      console.log(`   组织: ${this.ipinfo.as_name || this.ipinfo.org}`);
      if (this.ipinfo.latitude && this.ipinfo.longitude) {
        console.log(
          `   坐标: ${this.ipinfo.latitude}, ${this.ipinfo.longitude}`,
        );
      }
      if (this.ipinfo.time_zone) {
        console.log(`   时区: ${this.ipinfo.time_zone}`);
      }
    } else {
      console.error("❌ 所有IP信息获取都失败了");
      // 设置默认值，确保程序能继续运行
      this.ipinfo = {
        ip: "unknown",
        country: "unknown",
        country_code: "unknown",
        asn: "unknown",
        as_name: "unknown",
        source: "failed",
        error: "所有IP信息API都失败了",
      };
    }

    return this.ipinfo;
  }

  /**
   * 将IP信息格式化为Markdown字符串
   */
  formatAsMarkdown() {
    const timestamp = new Date().toLocaleString("zh-CN");

    let markdown = `## 🌐 当前测试环境信息

- **获取时间**: ${timestamp}
- **IP地址**: ${this.ipinfo.ip}
- **国家/地区**: ${this.ipinfo.country} (${this.ipinfo.country_code})
- **ASN**: ${this.ipinfo.asn}
- **网络组织**: ${this.ipinfo.as_name || this.ipinfo.org || "N/A"}
- **网络域名**: ${this.ipinfo.as_domain || "N/A"}`;

    if (this.ipinfo.continent) {
      markdown += `\n- **大洲**: ${this.ipinfo.continent} (${this.ipinfo.continent_code})`;
    }

    if (this.ipinfo.latitude && this.ipinfo.longitude) {
      markdown += `\n- **地理坐标**: ${this.ipinfo.latitude}, ${this.ipinfo.longitude}`;
    }

    if (this.ipinfo.time_zone) {
      markdown += `\n- **时区**: ${this.ipinfo.time_zone}`;
    }

    markdown += `\n- **数据源**: ${this.ipinfo.source}`;

    if (this.ipinfo.error) {
      markdown += `\n- ⚠️ **错误**: ${this.ipinfo.error}`;
    }

    markdown += `\n\n---\n\n`;

    return markdown;
  }

  /**
   * 将IP信息格式化为JSON对象
   */
  formatAsJSON() {
    return {
      timestamp: new Date().toISOString(),
      ip_info: this.ipinfo,
    };
  }
}

// 如果直接运行此脚本，则获取并显示IP信息
if (import.meta.main) {
  const fetcher = new IPInfoFetcher();
  fetcher.fetchIPInfo().then((ipInfo) => {
    console.log("\n" + "=".repeat(50));
    console.log("IP地址信息摘要");
    console.log("=".repeat(50));
    console.log(JSON.stringify(ipInfo, null, 2));
  });
}

export default IPInfoFetcher;
