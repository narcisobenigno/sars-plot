use serde::Deserialize;

#[derive(Debug, Deserialize, Eq, PartialEq)]
pub struct Row {
    #[serde(rename = "data de publicação")]
    pub data_de_publicacao: String,
    #[serde(rename = "UF")]
    pub uf: String,
    #[serde(rename = "Unidade da Federação")]
    pub unidade_da_federacao: String,
    #[serde(rename = "Tipo")]
    pub tipo: String,
    #[serde(rename = "dado")]
    pub dado: String,
    #[serde(rename = "escala")]
    pub escala: String,
    #[serde(rename = "Ano epidemiológico")]
    pub ano_epidemiologico: String,
    #[serde(rename = "Semana epidemiológica")]
    pub semana_epidemiologica: String,
    #[serde(rename = "Situação do dado")]
    pub situacao_do_dado: String,
    #[serde(rename = "Casos semanais reportados até a última atualização")]
    pub casos_semanais_reportados_ate_a_ultima_atualizacao: String,
    #[serde(rename = "limite inferior da estimativa")]
    pub limite_inferior_da_estimativa: String,
    #[serde(rename = "casos estimados")]
    pub casos_estimados: String,
    #[serde(rename = "média móvel")]
    pub media_movel: String,
    #[serde(rename = "limite superior da estimativa")]
    pub limite_superior_da_estimativa: String,
    #[serde(rename = "Percentual em relação ao país")]
    pub percentual_em_relacao_ao_pais: String,
    #[serde(rename = "População")]
    pub populacao: String,
    #[serde(rename = "limiar pré-epidêmico")]
    pub limiar_pre_epidemico: String,
    #[serde(rename = "intensidade alta")]
    pub intensidade_alta: String,
    #[serde(rename = "intensidade muito alta")]
    pub intensidade_muito_alta: String,
    #[serde(rename = "nível semanal")]
    pub nivel_semanal: String,
    #[serde(rename = "nível por média móvel")]
    pub nivel_por_media_movel: String,
}
