package resolver

import (
	"log"
	"testing"

	dns_experiment "github.com/masx200/http3-reverse-proxy-server-experiment/dns"
	"github.com/masx200/http3-reverse-proxy-server-experiment/generic"
	"github.com/miekg/dns"
)

func TestResolver9(t *testing.T) {
	x := "hello-word-worker-cloudflare.masx200.workers.dev"
	results, err := dns_experiment.DnsResolverMultipleServers(x, GetQueryCallbacks14(), func(dro *dns_experiment.DnsResolverOptions) {
		dro.DnsCache = DnsCache
	})

	if err != nil {
		log.Printf("Error: %v\n", err)
		t.Error(err)
		return
	}

	for _, result := range results {
		log.Println(x, result)
	}
}
func TestResolver28(t *testing.T) {
	x := "nextjs-doh-reverse-proxy.onrender.com"
	results, err := dns_experiment.DnsResolverMultipleServers(x, GetQueryCallbacks14(), func(dro *dns_experiment.DnsResolverOptions) {
		dro.DnsCache = DnsCache
	})

	if err != nil {
		log.Printf("Error: %v\n", err)
		t.Error(err)
		return
	}

	for _, result := range results {
		log.Println(x, result)
	}
}
func TestResolver37(t *testing.T) {
	x := "www.bilibili.com"
	results, err := dns_experiment.DnsResolverMultipleServers(x, GetQueryCallbacks14(), func(dro *dns_experiment.DnsResolverOptions) {
		dro.DnsCache = DnsCache
	})

	if err != nil {
		log.Printf("Error: %v\n", err)
		t.Error(err)
		return
	}

	for _, result := range results {
		log.Println(x, result)
	}
}
func GetQueryCallbacks14() generic.MapInterface[string, func(m *dns.Msg) (r *dns.Msg, err error)] {
	return generic.MapImplementFromMap(map[string]func(m *dns.Msg) (r *dns.Msg, err error){"https://deno-dns-over-https-server.g18uibxgnb.de5.net/": func(m *dns.Msg) (r *dns.Msg, err error) {
		return DohClient(m, "https://deno-dns-over-https-server.g18uibxgnb.de5.net/")
	}, "https://deno-dns-over-https-server.g18uibxgnb.de5.net/": func(m *dns.Msg) (r *dns.Msg, err error) {
		return DohClient(m, "https://deno-dns-over-https-server.g18uibxgnb.de5.net/")
	}, "https://deno-dns-over-https-server.g18uibxgnb.de5.net/": func(m *dns.Msg) (r *dns.Msg, err error) {
		return DoHTTP3Client(m, "https://deno-dns-over-https-server.g18uibxgnb.de5.net/")
	}, "https://deno-dns-over-https-server.g18uibxgnb.de5.net/": func(m *dns.Msg) (r *dns.Msg, err error) {
		return DoHTTP3Client(m, "https://deno-dns-over-https-server.g18uibxgnb.de5.net/")
	}, "quic://dns.alidns.com": func(m *dns.Msg) (r *dns.Msg, err error) {
		return DoQClient(m, "quic://dns.alidns.com")
	}, "quic://family.adguard-dns.com": func(m *dns.Msg) (r *dns.Msg, err error) {
		return DoQClient(m, "quic://family.adguard-dns.com")
	}, "tls://dot.pub": func(m *dns.Msg) (r *dns.Msg, err error) {
		return DoTClient(m, "tls://dot.pub")
	}, "tls://family.adguard-dns.com": func(m *dns.Msg) (r *dns.Msg, err error) {
		return DoTClient(m, "tls://family.adguard-dns.com")
	}})
}

func DoTClient(m *dns.Msg, s string) (r *dns.Msg, err error) {
	return dns_experiment.DoTClient(m, s)
}

func DoQClient(m *dns.Msg, s string) (r *dns.Msg, err error) {
	return dns_experiment.DoQClient(m, s)
}
func TestResolver42(t *testing.T) {
	x := "www.baidu.com"
	results, err := dns_experiment.DnsResolverMultipleServers(x, GetQueryCallbacks14(), func(dro *dns_experiment.DnsResolverOptions) {
		dro.DnsCache = DnsCache
	})

	if err != nil {
		log.Printf("Error: %v\n", err)
		t.Error(err)
		return
	}

	for _, result := range results {
		log.Println(x, result)
	}
}
func TestResolverMultipleServers2(t *testing.T) {
	x := "hello-word-worker-cloudflare.masx200.workers.dev"
	results, err := dns_experiment.DnsResolverMultipleServers(x, GetQueryCallbacks14(), func(dro *dns_experiment.DnsResolverOptions) {
		dro.DnsCache = DnsCache
	})

	if err != nil {
		log.Printf("Error: %v\n", err)
		t.Error(err)
		return
	}

	for _, result := range results {
		log.Println(x, result)
	}
}
func TestResolver424(t *testing.T) {
	x := "www.bilibili.com"
	results, err := dns_experiment.DnsResolverMultipleServers(x, GetQueryCallbacks14(), func(dro *dns_experiment.DnsResolverOptions) {
		dro.DnsCache = DnsCache
	})

	if err != nil {
		log.Printf("Error: %v\n", err)
		t.Error(err)
		return
	}

	for _, result := range results {
		log.Println(x, result)
	}
}
func TestResolverMultipleServers24(t *testing.T) {
	x := "hello-word-worker-cloudflare.masx200.workers.dev"
	results, err := dns_experiment.DnsResolverMultipleServers(x, GetQueryCallbacks14(), func(dro *dns_experiment.DnsResolverOptions) {
		dro.DnsCache = DnsCache
	})

	if err != nil {
		log.Printf("Error: %v\n", err)
		t.Error(err)
		return
	}

	for _, result := range results {
		log.Println(x, result)
	}
}
func TestResolverMultipleServers234(t *testing.T) {
	x := "www.fastly.com"
	results, err := dns_experiment.DnsResolverMultipleServers(x, GetQueryCallbacks14(), func(dro *dns_experiment.DnsResolverOptions) {
		dro.DnsCache = DnsCache
	})

	if err != nil {
		log.Printf("Error: %v\n", err)
		t.Error(err)
		return
	}

	for _, result := range results {
		log.Println(x, result)
	}
}
