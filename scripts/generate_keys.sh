#!/bin/bash
set -e

if [ "$#" -ne 1 ]; then
	echo "Please provide the number of initial validators!"
	exit 1
fi

# Copy paste your mnemonic here.
SECRET="lunar slab casual enlist cabin hover patient afford north express copy excite"

generate_account_id() {
	subkey inspect ${3:-} ${4:-} "$SECRET//$1//$2" | grep "Account ID" | awk '{ print $3 }'
}

generate_address() {
	subkey inspect ${3:-} ${4:-} "$SECRET//$1//$2" | grep "SS58 Address" | awk '{ print $3 }'
}

generate_public_key() {
	subkey inspect ${3:-} ${4:-} "$SECRET//$1//$2" | grep "Public" | awk '{ print $4 }'
}

generate_address_and_account_id() {
	ACCOUNT=$(generate_account_id $1 $2 $3)
	ADDRESS=$(generate_address $1 $2 $3)
	if ${4:-false}; then
		FUNC="hex2array_unchecked"
		INTO=".unchecked_into()"
	else
		FUNC="hex_n_into_unchecked"
		INTO=""
	fi

	printf "// $ADDRESS\narray_bytes::$FUNC(\"${ACCOUNT#'0x'}\")$INTO,"
}

V_NUM=$1

AUTHORITIES=""

for i in $(seq 1 $V_NUM); do
	AUTHORITIES+="(\n"
	AUTHORITIES+="$(generate_address_and_account_id $i stash)\n"
	AUTHORITIES+="$(generate_address_and_account_id $i controller)\n"
	AUTHORITIES+="$(generate_address_and_account_id $i grandpa '--scheme ed25519' true)\n"
	AUTHORITIES+="$(generate_address_and_account_id $i babe '--scheme sr25519' true)\n"
	AUTHORITIES+="$(generate_address_and_account_id $i im_online '--scheme sr25519' true)\n"
	AUTHORITIES+="$(generate_address_and_account_id $i authority_discovery '--scheme sr25519' true)\n"
	AUTHORITIES+="),\n"
done

printf "$AUTHORITIES"